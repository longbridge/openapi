mod server;

use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use longport::{Config, QuoteContext, TradeContext};
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_mcpserver::{McpServer, sse::sse_endpoint, stdio::stdio};
use server::Longport;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[derive(Parser)]
struct Cli {
    /// Use SSE transport
    #[clap(long)]
    sse: bool,
    /// Bind address for the SSE server.
    #[clap(long, default_value = "127.0.0.1:8000")]
    bind: String,
    /// Log directory
    #[clap(long)]
    log_dir: Option<PathBuf>,
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

    if !cli.sse {
        tracing::info!("Starting MCP server with stdio transport");
        let server = McpServer::new().tools(Longport::new(quote_context, trade_context));
        stdio(server).await?;
    } else {
        tracing::info!("Starting MCP server with SSE transport");
        let listener = TcpListener::bind(&cli.bind);
        let app = Route::new()
            .at(
                "/sse",
                sse_endpoint(move || {
                    let tools = Longport::new(quote_context.clone(), trade_context.clone());
                    McpServer::new().tools(tools)
                }),
            )
            .with(Cors::new());
        Server::new(listener).run(app).await?;
    }

    Ok(())
}
