# Rust MCP Server - IP Info Query Service

A Model Context Protocol (MCP) server implemented in Rust using the official rmcp SDK, providing public IP information query functionality.

## Features

- ✅ Get public IP address
- ✅ Query IP geolocation (city, region, country)
- ✅ Get ISP information
- ✅ Display timezone information
- ✅ Provide latitude and longitude coordinates

## Quick Start

### Configure MCP Client

#### Claude Desktop Configuration

**Using npx (recommended):**

Edit the configuration file:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"]
    }
  }
}
```


#### Cursor Configuration

**Using npx (recommended):**
```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"]
    }
  }
}
```

### Restart Claude Desktop

After configuration, completely quit and restart Claude Desktop.

### Using the Tool

Simply ask in Claude Desktop:

```
Please get the current machine's public IP information
```

## Enable Verbose Logging 🔍

If you need to view detailed runtime logs (for debugging or understanding program operation), you can add an `env` field to the configuration:

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"],
      "env": {
        "RUST_LOG": "debug"
      }
    }
  }
}
```
