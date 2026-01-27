# Rust MCP Server - IP 信息查询服务

一个使用 Rust 和官方 rmcp SDK 实现的 Model Context Protocol (MCP) 服务器，提供公网 IP 信息查询功能。

## 功能特性

- ✅ 获取公网 IP 地址
- ✅ 查询 IP 地理位置（城市、地区、国家）
- ✅ 获取 ISP 信息
- ✅ 显示时区信息
- ✅ 提供经纬度坐标

## 快速开始

### 配置 MCP 客户端

#### Claude Desktop 配置

**使用 npx（推荐）：**

编辑配置文件：
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


#### Cursor 配置

**使用 npx（推荐）：**
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

### 重启 Claude Desktop

配置完成后，完全退出并重新启动 Claude Desktop。

### 使用工具

在 Claude Desktop 中直接询问：

```
请帮我获取当前机器的公网 IP 信息
```

## 启用详细日志 🔍

如果需要查看详细的运行日志(用于调试或了解程序运行情况),可以在配置中添加 `env` 字段:

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
