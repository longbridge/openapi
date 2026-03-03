mod oauth;
mod server;

use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use longport::{Config, QuoteContext, TradeContext};
use oauth::{AuthenticatedContext, AuthorizationServerMetadata, BearerAuthMiddleware};
use poem::{
    EndpointExt, IntoResponse, Request, Response, Route, Server,
    handler,
    http::StatusCode,
    listener::TcpListener,
    middleware::Cors,
    web::{Data, Json},
};
use poem_mcpserver::{McpServer, stdio::stdio, streamable_http};
use server::Longport;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(about = "LongPort MCP server")]
struct Cli {
    /// Use Streamable-HTTP transport instead of STDIO
    #[clap(long)]
    http: bool,

    /// Bind address for the HTTP server
    #[clap(long, default_value = "127.0.0.1:8000")]
    bind: String,

    /// Enable OAuth 2.0 Bearer token authentication for the HTTP transport.
    ///
    /// When set, every request to the MCP HTTP endpoint must carry an
    /// `Authorization: Bearer <access_token>` header.  The access token must
    /// be a valid LongPort OAuth 2.0 token obtained via the authorization code
    /// flow.
    ///
    /// Implies --http.  When used, LONGPORT_APP_KEY / LONGPORT_APP_SECRET /
    /// LONGPORT_ACCESS_TOKEN environment variables are **not** required because
    /// the per-request credentials are derived from the Bearer token.
    #[clap(long)]
    oauth: bool,

    /// Log directory
    #[clap(long)]
    log_dir: Option<PathBuf>,

    /// Read-only mode: prevents submitting orders to the exchange
    #[clap(long, default_value_t = false)]
    readonly: bool,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    if let Some(log_dir) = cli.log_dir {
        let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "longport-mcp.log");
        tracing_subscriber::fmt()
            .with_writer(file_appender)
            .with_ansi(false)
            .init();
    }

    let readonly = cli.readonly;

    if cli.oauth || cli.http {
        if cli.oauth {
            run_http_oauth(cli.bind, readonly).await?;
        } else {
            run_http_env(cli.bind, readonly).await?;
        }
    } else {
        run_stdio(readonly).await?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Transport implementations
// ---------------------------------------------------------------------------

/// Run with STDIO transport using credentials from environment variables.
async fn run_stdio(readonly: bool) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting MCP server with STDIO transport");

    let config = Arc::new(
        Config::from_env()
            .inspect_err(|err| tracing::error!(error = %err, "failed to load config"))?
            .dont_print_quote_packages(),
    );

    let (quote_ctx, _) = QuoteContext::try_new(config.clone()).await?;
    let (trade_ctx, _) = TradeContext::try_new(config.clone()).await?;
    let server = build_mcp_server(quote_ctx, trade_ctx, readonly);

    stdio(server).await?;
    Ok(())
}

/// Run with Streamable-HTTP transport using credentials from environment
/// variables (no OAuth enforcement).
async fn run_http_env(
    bind: String,
    readonly: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(
        bind = %bind,
        "Starting MCP server with Streamable-HTTP transport (env credentials)"
    );

    let config = Arc::new(
        Config::from_env()
            .inspect_err(|err| tracing::error!(error = %err, "failed to load config"))?
            .dont_print_quote_packages(),
    );
    let (quote_ctx, _) = QuoteContext::try_new(config.clone()).await?;
    let (trade_ctx, _) = TradeContext::try_new(config.clone()).await?;

    let listener = TcpListener::bind(&bind);
    let app = Route::new()
        .at(
            "/",
            streamable_http::endpoint(move |_| {
                build_mcp_server(quote_ctx.clone(), trade_ctx.clone(), readonly)
            }),
        )
        .with(Cors::new());

    Server::new(listener).run(app).await?;
    Ok(())
}

/// Run with Streamable-HTTP transport and OAuth 2.0 Bearer token enforcement.
///
/// Each HTTP request is authenticated via [`BearerAuthMiddleware`].  On a
/// successful authentication the middleware injects an
/// [`AuthenticatedContext`] that contains a per-request [`Config`] built from
/// the caller's Bearer token.  A fresh pair of [`QuoteContext`] /
/// [`TradeContext`] is created for every MCP session.
///
/// The `/.well-known/oauth-authorization-server` route serves RFC 8414
/// discovery metadata.
async fn run_http_oauth(
    bind: String,
    readonly: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(
        bind = %bind,
        "Starting MCP server with Streamable-HTTP transport and OAuth 2.0 enforcement"
    );

    // Derive the public base URL from the bind address.  In production this
    // should be the externally reachable URL; the bind address is used as a
    // sensible default for local / dev scenarios.
    let base_url = if bind.starts_with("0.0.0.0:") || bind.starts_with("[::]") {
        format!("http://localhost:{}", bind.split(':').last().unwrap_or("8000"))
    } else {
        format!("http://{bind}")
    };

    let listener = TcpListener::bind(&bind);
    let app = Route::new()
        // RFC 8414 discovery endpoint — no auth required
        .at(
            "/.well-known/oauth-authorization-server",
            poem::get(oauth_metadata_handler),
        )
        .data(base_url)
        // MCP endpoint — auth required
        .at(
            "/",
            streamable_http::endpoint(move |req: &Request| {
                // Retrieve the per-request AuthenticatedContext injected by
                // BearerAuthMiddleware and build fresh contexts from it.
                let auth_ctx = req
                    .extensions()
                    .get::<AuthenticatedContext>()
                    .cloned();

                let config = match auth_ctx {
                    Some(ctx) => ctx.config(),
                    None => {
                        // Middleware should have rejected the request already;
                        // this branch is a safety net.
                        tracing::error!("BearerAuthMiddleware did not inject AuthenticatedContext");
                        // Return a dummy server that immediately errors — the
                        // connection will be closed by the client on first tool call.
                        let dummy_config = Arc::new(Config::new("", "", ""));
                        dummy_config
                    }
                };

                let readonly = readonly;
                async move {
                    let config = config.dont_print_quote_packages();
                    let config = Arc::new(config);
                    let (quote_ctx, _) = QuoteContext::try_new(config.clone()).await?;
                    let (trade_ctx, _) = TradeContext::try_new(config).await?;
                    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(
                        build_mcp_server(quote_ctx, trade_ctx, readonly),
                    )
                }
            }),
        )
        .with(BearerAuthMiddleware)
        .with(Cors::new());

    Server::new(listener).run(app).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Serve RFC 8414 Authorization Server Metadata.
#[handler]
async fn oauth_metadata_handler(base_url: Data<&String>) -> Response {
    let metadata = AuthorizationServerMetadata::new(base_url.0);
    match serde_json::to_vec(&metadata) {
        Ok(body) => Response::builder()
            .status(StatusCode::OK)
            .content_type("application/json")
            .body(body),
        Err(e) => {
            tracing::error!(error = %e, "failed to serialize OAuth metadata");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .finish()
        }
    }
}

// ---------------------------------------------------------------------------
// MCP server factory
// ---------------------------------------------------------------------------

fn build_mcp_server(
    quote_context: QuoteContext,
    trade_context: TradeContext,
    readonly: bool,
) -> McpServer<Longport> {
    let mut server = McpServer::new().tools(Longport::new(quote_context, trade_context));
    if readonly {
        server = server.disable_tools(["submit_order"]);
    }
    server
}
