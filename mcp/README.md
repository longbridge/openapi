# LongPort MCP

A [MCP](https://modelcontextprotocol.io/introduction) server implementation for [LongPort OpenAPI](https://open.longportapp.com), provides real-time stock market data, provides AI access analysis and trading capabilities through MCP.

## Documentation

- LongPort OpenAPI: https://open.longportapp.com/en/
- SDK docs: https://longportapp.github.io/openapi

## Features

- Trading - Create, amend, cancel orders, query today's/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds

## Installation

### macOS or Linux

Run script to install:

```bash
curl -sSL https://raw.githubusercontent.com/longportapp/openapi/refs/heads/main/mcp/install | bash
```

### Windows

Download the latest binary from the [Releases](https://github.com/longportapp/openapi/releases/tag/longport-mcp-0.1.0) page.

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

To configure LongPort MCP in Cursor:

- Open Cursor Settings
- Go to Features > MCP Servers
- Click `+ Add New MCP Server`
- Enter the following:
  - Name: `longport-mcp` (or your preferred name)
  - Type: `command`
  - Command: `env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp`

If you are using Windows, replace command with `cmd /c "set LONGPORT_APP_KEY=your-app-key && set LONGPORT_APP_SECRET=your-app-secret && set LONGPORT_ACCESS_TOKEN=your-access-token && longport-mcp"`

Or use this config:

```json
{
  "mcpServers": {
    "longport-mcp": {
      "command": "/usr/local/bin/longport-mcp",
      "env": {
        "LONGPORT_APP_KEY": "your-app-key",
        "LONGPORT_APP_SECRET": "your-app-secret",
        "LONGPORT_ACCESS_TOKEN": "your-access-token"
      }
    }
  }
}
```

### Use in Cherry Studio

To configure LongPort MCP in Cherry Studio:

- Go to Settings > MCP Servers
- Click `+ Add Server`
- Enter the following:
  - Name: `longport-mcp` (or your preferred name)
  - Type: `STDIO`
  - Command: `env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp`

If you are using Windows, replace command with `cmd /c "set LONGPORT_APP_KEY=your-app-key && set LONGPORT_APP_SECRET=your-app-secret && set LONGPORT_ACCESS_TOKEN=your-access-token && longport-mcp"`

## Running as an HTTP server

```bash
env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp --http
```

Default bind address is `127.0.0.1:8000`, you can change it by using the `--bind` flag:

```bash
longport-mcp --http --bind 127.0.0.1:3000
```

## Running with OAuth 2.0 authentication

The `--oauth` flag enables OAuth 2.0 Bearer token authentication for the HTTP transport.  In this mode:

- The server does **not** require `LONGPORT_APP_KEY` / `LONGPORT_APP_SECRET` / `LONGPORT_ACCESS_TOKEN` environment variables.
- Each request must carry a valid `Authorization: Bearer <access_token>` header.
- The access token must be a LongPort OAuth 2.0 access token obtained via the authorization code flow.
- Each MCP session operates under the requesting user's own LongPort credentials.

```bash
longport-mcp --oauth
```

With a custom bind address:

```bash
longport-mcp --oauth --bind 0.0.0.0:8080
```

### OAuth 2.0 discovery

When running with `--oauth`, the server exposes an RFC 8414 discovery document at:

```
GET /.well-known/oauth-authorization-server
```

This allows AI clients that implement OAuth 2.0 dynamic discovery (e.g. Claude Desktop, Cursor) to automatically configure themselves without manual credential entry.

Example response:

```json
{
  "issuer": "http://127.0.0.1:8000",
  "authorization_endpoint": "https://openapi.longbridgeapp.com/oauth2/authorize",
  "token_endpoint": "https://openapi.longbridgeapp.com/oauth2/token",
  "revocation_endpoint": "https://openapi.longbridgeapp.com/oauth2/revoke",
  "response_types_supported": ["code"],
  "grant_types_supported": ["authorization_code", "refresh_token"],
  "code_challenge_methods_supported": ["S256"]
}
```

### Obtaining an OAuth 2.0 access token

Use the `longport-oauth` crate or the LongPort developer portal to initiate the authorization code flow.  Once you have an access token, configure your MCP client:

```json
{
  "mcpServers": {
    "longport-mcp": {
      "url": "http://127.0.0.1:8000",
      "headers": {
        "Authorization": "Bearer your-oauth-access-token"
      }
    }
  }
}
```

## Configuration

### Readonly mode

To run the server in read-only mode, set the flag `--readonly`:

```bash
longport-mcp --readonly
```

This will prevent the server from submitting orders to the exchange.

### Enable logging

To enable logging, set the flag `--log-dir` to the directory where you want to store the logs:

```bash
longport-mcp --log-dir /path/to/log/dir
```
