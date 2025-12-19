# IP Info MCP Server

一个符合 [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) 规范的服务器，用于获取机器的公网IP信息。

## 功能特性

- 符合MCP协议规范（2024-11-05版本）
- 通过stdin/stdout进行JSON-RPC 2.0通信
- 提供`get_ip_info`工具获取IP地址、地理位置、ISP等信息
- 使用ipinfo.io API作为数据源

## 项目结构

```
src/
├── main.rs      # 程序入口，启动MCP server
├── lib.rs       # IP信息结构体和获取逻辑
├── mcp.rs       # MCP协议消息结构定义
└── server.rs    # MCP server核心实现
```

## 构建项目

```bash
cargo build --release
```

## 使用方法

### 作为MCP Server运行

MCP server通过stdin/stdout与客户端通信：

```bash
cargo run
```

### 支持的MCP方法

#### 1. initialize

初始化连接，获取服务器信息和能力。

**请求示例：**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "test-client",
      "version": "1.0.0"
    }
  }
}
```

**响应示例：**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "ip-info-server",
      "version": "0.1.0"
    }
  }
}
```

#### 2. tools/list

获取可用工具列表。

**请求示例：**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

**响应示例：**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "get_ip_info",
        "description": "获取当前机器的公网IP信息，包括IP地址、地理位置、ISP等",
        "inputSchema": {
          "type": "object",
          "properties": {},
          "required": []
        }
      }
    ]
  }
}
```

#### 3. tools/call

调用工具获取IP信息。

**请求示例：**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "get_ip_info"
  }
}
```

**响应示例：**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "IP Information:\n  IP Address: xxx.xxx.xxx.xxx\n  City: Los Angeles\n  Region: California\n  Country: US\n  Location: Latitude=34.0522, Longitude=-118.2437\n  Organization: AS25820 IT7 Networks Inc\n  Timezone: America/Los_Angeles\n"
      }
    ]
  }
}
```

## 测试

运行测试脚本验证所有功能：

```bash
./test_mcp.sh
```

或者手动测试单个方法：

```bash
# 测试 initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' | cargo run

# 测试 tools/list
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | cargo run

# 测试 tools/call
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_ip_info"}}' | cargo run
```

## 在Claude Desktop中配置

在Claude Desktop的配置文件中添加此MCP server：

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`

**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "/path/to/rust-mcp-client/target/release/rust-mcp-client"
    }
  }
}
```

配置后重启Claude Desktop即可使用`get_ip_info`工具。

## 依赖项

- `tokio` - 异步运行时
- `reqwest` - HTTP客户端
- `serde` / `serde_json` - JSON序列化/反序列化

## 协议版本

本项目实现了MCP协议版本 `2024-11-05`。

## 许可证

MIT
