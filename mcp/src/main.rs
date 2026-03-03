mod oauth;
mod server;

use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use longport::{Config, QuoteContext, TradeContext};
use oauth::{AuthenticatedContext, AuthorizationServerMetadata, BearerAuthMiddleware};
use poem::{
    EndpointExt, Request, Response, Route, Server,
    handler,
    http::StatusCode,
    listener::TcpListener,
    middleware::Cors,
    web::Data,
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
    /// Use Streamable-HTTP transport instead of STDIO.
    #[clap(long)]
    http: bool,

    /// Bind address for the HTTP server.
    #[clap(long, default_value = "127.0.0.1:8000")]
    bind: String,

    /// Enable OAuth 2.0 Bearer token authentication for the HTTP transport.
    ///
    /// Every request to the MCP HTTP endpoint must carry a valid
    /// `Authorization: Bearer <access_token>` header.  The token must be a
    /// LongPort OAuth 2.0 access token obtained via the authorization code
    /// flow (see `longport-oauth` crate or the LongPort developer portal).
    ///
    /// Implies --http.  Environment variables LONGPORT_APP_KEY /
    /// LONGPORT_APP_SECRET / LONGPORT_ACCESS_TOKEN are not required because
    /// per-request credentials are derived from the Bearer token.
    #[clap(long)]
    oauth: bool,

    /// Log directory.
    #[clap(long)]
    log_dir: Option<PathBuf>,

    /// Read-only mode: prevents submitting orders to the exchange.
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

    if cli.oauth {
        run_http_oauth(cli.bind, readonly).await?;
    } else if cli.http {
        run_http_env(cli.bind, readonly).await?;
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
    let (trade_ctx, _) = TradeContext::try_new(config).await?;

    stdio(build_mcp_server(quote_ctx, trade_ctx, readonly)).await?;
    Ok(())
}

/// Run with Streamable-HTTP transport, credentials from environment variables.
///
/// All sessions share the same LongPort credentials.  No OAuth enforcement.
async fn run_http_env(bind: String, readonly: bool) -> Result<(), Box<dyn std::error::Error>> {
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
    let (trade_ctx, _) = TradeContext::try_new(config).await?;

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
/// Authentication flow per request:
/// 1. [`BearerAuthMiddleware`] validates the `Authorization: Bearer <token>`
///    header against the LongPort userinfo endpoint.
/// 2. On success, it injects [`AuthenticatedContext`] into request extensions.
/// 3. The `streamable_http` endpoint factory reads the context, constructs a
///    per-session [`Config`] from the verified token, and creates fresh
///    [`QuoteContext`] / [`TradeContext`] instances for that session.
///
/// The `/.well-known/oauth-authorization-server` route (RFC 8414) is exempt
/// from authentication and allows AI clients to auto-discover the
/// authorization server.
async fn run_http_oauth(bind: String, readonly: bool) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(
        bind = %bind,
        "Starting MCP server with Streamable-HTTP transport and OAuth 2.0 enforcement"
    );

    let base_url = derive_public_base_url(&bind);

    let listener = TcpListener::bind(&bind);
    let app = Route::new()
        // RFC 8414 discovery endpoint — intentionally unauthenticated (RFC 8414 §3).
        .at(
            "/.well-known/oauth-authorization-server",
            poem::get(oauth_metadata_handler),
        )
        .data(base_url)
        // MCP endpoint — every request must carry a valid Bearer token.
        // BearerAuthMiddleware has already validated the token and injected
        // AuthenticatedContext by the time this factory is called.
        .at(
            "/",
            streamable_http::endpoint(move |req: &Request| {
                // Clone the context out of the request before the async block so
                // the future does not borrow `req`.
                let auth_ctx = req.extensions().get::<AuthenticatedContext>().cloned();
                async move {
                    let auth_ctx = auth_ctx.ok_or(
                        "BearerAuthMiddleware did not inject AuthenticatedContext",
                    )?;
                    let config =
                        Arc::new(auth_ctx.build_config().dont_print_quote_packages());
                    let (quote_ctx, _) = QuoteContext::try_new(config.clone()).await?;
                    let (trade_ctx, _) = TradeContext::try_new(config).await?;
                    Ok::<McpServer<Longport>, Box<dyn std::error::Error + Send + Sync>>(
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

/// Serve RFC 8414 Authorization Server Metadata at
/// `/.well-known/oauth-authorization-server`.
///
/// This endpoint is intentionally unauthenticated per RFC 8414 §3.
#[handler]
async fn oauth_metadata_handler(base_url: Data<&String>) -> Response {
    let metadata = AuthorizationServerMetadata::new(base_url.0.as_str());
    match serde_json::to_vec(&metadata) {
        Ok(body) => Response::builder()
            .status(StatusCode::OK)
            .content_type("application/json")
            .body(body),
        Err(e) => {
            tracing::error!(error = %e, "failed to serialize OAuth 2.0 discovery metadata");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .finish()
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Derive a public HTTP base URL from a TCP bind address.
///
/// For wildcard binds (`0.0.0.0:PORT` or `[::]:PORT`) substitutes `localhost`
/// so that the RFC 8414 discovery document is usable in local dev environments.
pub(crate) fn derive_public_base_url(bind: &str) -> String {
    if bind.starts_with("0.0.0.0:") || bind.starts_with("[::]:") {
        let port = bind.rsplit(':').next().unwrap_or("8000");
        format!("http://localhost:{port}")
    } else {
        format!("http://{bind}")
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_public_base_url_wildcard_ipv4() {
        assert_eq!(
            derive_public_base_url("0.0.0.0:8000"),
            "http://localhost:8000"
        );
    }

    #[test]
    fn test_derive_public_base_url_wildcard_ipv6() {
        assert_eq!(
            derive_public_base_url("[::]:9000"),
            "http://localhost:9000"
        );
    }

    #[test]
    fn test_derive_public_base_url_loopback() {
        assert_eq!(
            derive_public_base_url("127.0.0.1:8000"),
            "http://127.0.0.1:8000"
        );
    }

    #[test]
    fn test_derive_public_base_url_named_host() {
        assert_eq!(
            derive_public_base_url("mcp.example.com:443"),
            "http://mcp.example.com:443"
        );
    }

    #[test]
    fn test_derive_public_base_url_wildcard_preserves_port() {
        assert_eq!(
            derive_public_base_url("0.0.0.0:3333"),
            "http://localhost:3333"
        );
    }
}
