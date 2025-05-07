mod server;

use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use longport::{Config, QuoteContext, TradeContext};
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_mcpserver::{McpServer, stdio::stdio, streamable_http};
use server::Longport;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[derive(Parser)]
struct Cli {
    /// Use Streamable-HTTP transport
    #[clap(long)]
    http: bool,
    /// Bind address for the SSE server.
    #[clap(long, default_value = "127.0.0.1:8000")]
    bind: String,
    /// Log directory
    #[clap(long)]
    log_dir: Option<PathBuf>,
    /// Read-only mode
    ///
    /// This mode is used to prevent submitting orders to the exchange.
    #[clap(long, default_value_t = false)]
    readonly: bool,
}

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

    let config = Arc::new(
        Config::from_env()
            .inspect_err(|err| tracing::error!(error = %err, "failed to load config"))?
            .dont_print_quote_packages(),
    );
    let (quote_context, _) = QuoteContext::try_new(config.clone()).await?;
    let (trade_context, _) = TradeContext::try_new(config.clone()).await?;
    let readonly = cli.readonly;

    if !cli.http {
        tracing::info!("Starting MCP server with stdio transport");
        let server = create_mcp_server(quote_context, trade_context, readonly);
        stdio(server).await?;
    } else {
        tracing::info!(
            "Starting MCP server with Streamable-HTTP transport, listening on {}",
            cli.bind
        );
        let listener = TcpListener::bind(&cli.bind);
        let app = Route::new()
            .at(
                "/",
                streamable_http::endpoint(move |_| {
                    create_mcp_server(quote_context.clone(), trade_context.clone(), readonly)
                }),
            )
            .with(Cors::new());
        Server::new(listener).run(app).await?;
    }

    Ok(())
}

fn create_mcp_server(
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
