# Longbridge MCP

A [MCP](https://modelcontextprotocol.io/introduction) server implementation for [Longbridge OpenAPI](https://open.longbridge.com), provides real-time stock market data, provides AI access analysis and trading capabilities through MCP.

## Documentation

- Longbridge OpenAPI: https://open.longbridge.com/en/
- SDK docs: https://longbridge.github.io/openapi

## Features

- Trading - Create, amend, cancel orders, query today’s/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds

## Installation

### macOS or Linux

Run script to install:

```bash
curl -sSL https://raw.githubusercontent.com/longbridge/openapi/refs/heads/main/mcp/install | bash
```

### Windows

Download the latest binary from the [Releases](https://github.com/longbridge/openapi/releases/tag/longbridge-mcp-0.1.0) page.

## Example Prompts

Once you done server setup, and connected, you can talk with AI:

- What's the current price of AAPL and TSLA stock?
- How has Tesla performed over the past month?
- Show me the current values of major market indices.
- What's the stock price history for TSLA, AAPL over the last year?
- Compare the performance of TSLA, AAPL and NVDA over the past 3 months.
- Generate a portfolio performance chart for my holding stocks, and return me with data table and pie chart (Just return result no code).
- Check the price of the stocks I hold today, and if they fall/rise by more than 3%, sell(If fall, buy if rise) 1/3 at the market price.

## Usage

### Use in Cursor

To configure Longbridge MCP in Cursor:

- Open Cursor Settings
- Go to Features > MCP Servers
- Click `+ Add New MCP Server`
- Enter the following:
  - Name: `longbridge-mcp` (or your preferred name)
  - Type: `command`
  - Command: `env LONGBRIDGE_APP_KEY=your-app-key LONGBRIDGE_APP_SECRET=your-app-secret LONGBRIDGE_ACCESS_TOKEN=your-access-token longbridge-mcp`

If you are using Windows, replace command with `cmd /c "set LONGBRIDGE_APP_KEY=your-app-key && set LONGBRIDGE_APP_SECRET=your-app-secret && set LONGBRIDGE_ACCESS_TOKEN=your-access-token && longbridge-mcp"`

Or use this config:

```json
{
  "mcpServers": {
    "longbridge-mcp": {
      "command": "/usr/local/bin/longbridge-mcp",
      "env": {
        "LONGBRIDGE_APP_KEY": "your-app-key",
        "LONGBRIDGE_APP_SECRET": "your-app-secret",
        "LONGBRIDGE_ACCESS_TOKEN": "your-access-token"
      }
    }
  }
}
```

### Use in Cherry Studio

To configure Longbridge MCP in Cherry Studio:

- Go to Settings > MCP Servers
- Click `+ Add Server`
- Enter the following:
  - Name: `longbridge-mcp` (or your preferred name)
  - Type: `STDIO`
  - Command: `env LONGBRIDGE_APP_KEY=your-app-key LONGBRIDGE_APP_SECRET=your-app-secret LONGBRIDGE_ACCESS_TOKEN=your-access-token longbridge-mcp`

If you are using Windows, replace command with `cmd /c "set LONGBRIDGE_APP_KEY=your-app-key && set LONGBRIDGE_APP_SECRET=your-app-secret && set LONGBRIDGE_ACCESS_TOKEN=your-access-token && longbridge-mcp"`

## Running as a SSE server

```bash
env LONGBRIDGE_APP_KEY=your-app-key LONGBRIDGE_APP_SECRET=your-app-secret LONGBRIDGE_ACCESS_TOKEN=your-access-token longbridge-mcp --sse
```

Default bind address is `127.0.0.1:8000`, you can change it by using the `--bind` flag:

```bash
longbridge-mcp --sse --bind 127.0.0.1:3000
```

## Configuration

### Readonly mode

To run the server in read-only mode, set the flag `--readonly`:

```bash
longbridge-mcp --readonly
```

This will prevent the server from submitting orders to the exchange.

### Enable logging

To enable logging, set the flag `--log-dir` to the directory where you want to store the logs:

```bash
longbridge-mcp --log-dir /path/to/log/dir
```
