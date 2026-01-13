# MCP (Model Context Protocol) 协议详解

## 概述

MCP (Model Context Protocol) 是一个基于 JSON-RPC 2.0 的标准协议，用于 AI 助手（如 Claude）与外部服务之间的通信。它定义了一套固定的方法名和通信规范。

**当前协议版本**: `2024-11-05`

## 协议特点

- 基于 **JSON-RPC 2.0** 标准
- 使用 **stdin/stdout** 或 **HTTP** 传输
- 方法名都是**固定的**，由协议规范定义
- 支持工具、资源、提示词等多种能力

---

## 标准方法列表

### 1. 核心生命周期方法

#### `initialize`
**用途**: 初始化连接，协商协议版本和能力

**请求示例**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "clientInfo": {
      "name": "claude-desktop",
      "version": "1.0.0"
    }
  }
}
```

**响应示例**:
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
    },
    "instructions": "提供公网 IP 信息查询服务"
  }
}
```

#### `initialized`
**用途**: 通知服务器初始化完成（通知类型，无需响应）

#### `ping`
**用途**: 心跳检测，保持连接活跃

---

### 2. 工具相关方法

#### `tools/list`
**用途**: 获取服务器提供的所有工具列表

**请求示例**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

**响应示例**:
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

#### `tools/call`
**用途**: 调用指定的工具

**请求示例**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "get_ip_info",
    "arguments": {}
  }
}
```

**响应示例**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "IP Information:\n  IP Address: 203.0.113.42\n  City: Beijing\n  ..."
      }
    ]
  }
}
```

---

### 3. 资源相关方法

| 方法名 | 用途 | 说明 |
|--------|------|------|
| `resources/list` | 列出资源 | 返回服务器提供的所有资源列表 |
| `resources/read` | 读取资源 | 获取指定资源的内容 |
| `resources/templates/list` | 列出资源模板 | 返回带参数的资源模板 |
| `resources/subscribe` | 订阅资源 | 订阅资源变更通知 |
| `resources/unsubscribe` | 取消订阅 | 取消资源订阅 |

**资源示例**:
```json
{
  "uri": "file:///path/to/file.txt",
  "name": "配置文件",
  "description": "系统配置文件",
  "mimeType": "text/plain"
}
```

---

### 4. 提示词相关方法

| 方法名 | 用途 | 说明 |
|--------|------|------|
| `prompts/list` | 列出提示词 | 返回可用的提示词模板列表 |
| `prompts/get` | 获取提示词 | 获取指定提示词的完整内容 |

**提示词示例**:
```json
{
  "name": "code_review",
  "description": "代码审查提示词",
  "arguments": [
    {
      "name": "language",
      "description": "编程语言",
      "required": true
    }
  ]
}
```

---

### 5. 其他方法

| 方法名 | 用途 | 说明 |
|--------|------|------|
| `logging/setLevel` | 设置日志级别 | 配置服务器日志输出级别（debug/info/warning/error） |
| `completion/complete` | 自动完成 | 提供参数或提示词的自动完成建议 |

---

## 在 Rust rmcp SDK 中的使用

### 基本结构

```rust
use rmcp::*;
use rmcp::model::{CallToolResult, ServerInfo, ServerCapabilities, Content};
use rmcp::handler::server::router::tool::ToolRouter;

#[derive(Clone)]
pub struct MyServer {
    // 必需：tool_router 字段
    tool_router: ToolRouter<Self>,
}

impl MyServer {
    pub fn new() -> Self {
        Self {
            // 调用宏自动生成的方法
            tool_router: Self::tool_router(),
        }
    }
}
```

### 定义工具

```rust
// #[tool_router] 宏会自动处理 tools/list 和 tools/call
#[tool_router]
impl MyServer {
    /// 工具方法 - 会自动注册到 MCP
    #[tool(description = "获取当前机器的公网IP信息")]
    pub async fn get_ip_info(&self) -> Result<CallToolResult, ErrorData> {
        // 实现工具逻辑
        let content = Content::text("IP: 203.0.113.42");
        Ok(CallToolResult::success(vec![content]))
    }

    /// 带参数的工具
    #[tool(description = "查询指定城市的天气")]
    pub async fn get_weather(
        &self,
        #[tool(param)]
        #[schemars(description = "城市名称")]
        city: String
    ) -> Result<CallToolResult, ErrorData> {
        // 实现逻辑
        let content = Content::text(format!("{}的天气: 晴朗", city));
        Ok(CallToolResult::success(vec![content]))
    }
}
```

### 实现服务器处理器

```rust
// #[tool_handler] 宏会自动处理 initialize 等核心方法
#[tool_handler]
impl ServerHandler for MyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("我的 MCP 服务器说明".into()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()  // 启用工具能力
                .build(),
            ..Default::default()
        }
    }
}
```

### 启动服务器

```rust
use rmcp::transport::stdio;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = MyServer::new();
    let server = service.serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
```

---

## 宏的作用

### `#[tool_router]`
- 自动生成 `Self::tool_router()` 方法
- 为所有 `#[tool]` 标注的方法生成路由逻辑
- 自动生成 JSON Schema（从函数签名推导）

### `#[tool_handler]`
- 自动实现 `tools/list` 方法（列出所有工具）
- 自动实现 `tools/call` 方法（路由到具体工具）
- 自动实现 `initialize` 等核心协议方法

### `#[tool]`
- 标记一个方法为 MCP 工具
- 支持参数：
  - `description`: 工具描述
  - `name`: 自定义工具名（默认使用函数名）

---

## 错误处理

### 标准错误码

| 错误码 | 含义 | 说明 |
|--------|------|------|
| `-32700` | Parse error | JSON 解析错误 |
| `-32600` | Invalid Request | 无效的请求 |
| `-32601` | Method not found | 方法不存在 |
| `-32602` | Invalid params | 参数无效 |
| `-32603` | Internal error | 内部错误 |

### 在代码中使用

```rust
use rmcp::ErrorData;

// 返回错误
Err(ErrorData::internal_error("出错了", None))
Err(ErrorData::invalid_request("参数错误", None))
Err(ErrorData::parse_error("解析失败", None))
```

---

## 配置 Claude Desktop

### 配置文件位置
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`

### 配置示例

```json
{
  "mcpServers": {
    "my-server": {
      "command": "/path/to/your/mcp-server"
    },
    "with-args": {
      "command": "/path/to/server",
      "args": ["--config", "config.json"]
    },
    "with-env": {
      "command": "node",
      "args": ["server.js"],
      "env": {
        "API_KEY": "your-key"
      }
    }
  }
}
```

---

## 调试技巧

### 1. 手动测试服务器

```bash
# 测试 initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./your-mcp-server

# 测试 tools/list
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | ./your-mcp-server

# 测试 tools/call
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_ip_info","arguments":{}}}' | ./your-mcp-server
```

### 2. 查看日志

**Claude Desktop 日志位置**:
- macOS: `~/Library/Logs/Claude/`
- Windows: `%APPDATA%\Claude\logs\`

### 3. 使用 MCP Inspector

```bash
npx @modelcontextprotocol/inspector /path/to/your/server
```

这会启动一个 Web 界面，可以交互式测试你的 MCP 服务器。

---

## 完整示例项目

参考本项目的实现：

```
rust-mcp-client/
├── src/
│   ├── main.rs          # 服务器启动
│   ├── server.rs        # MCP 服务器实现
│   └── lib.rs           # 业务逻辑
├── Cargo.toml           # 依赖配置
└── docs/
    └── MCP_PROTOCOL.md  # 本文档
```

---

## 参考资料

- [MCP 官方规范](https://spec.modelcontextprotocol.io/specification/2024-11-05/)
- [rmcp Crate 文档](https://docs.rs/rmcp/latest/rmcp/)
- [官方 Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [MCP 服务器列表](https://github.com/modelcontextprotocol/servers)
- [Shuttle MCP 教程](https://www.shuttle.dev/blog/2025/07/18/how-to-build-a-stdio-mcp-server-in-rust)

---

## 常见问题

### Q: 可以自定义方法名吗？
**A**: 不可以。`initialize`、`tools/list`、`tools/call` 等方法名都是协议规范固定的，客户端和服务器必须遵循相同规范。

### Q: 如何添加多个工具？
**A**: 在同一个 `#[tool_router]` impl 块中添加多个 `#[tool]` 标注的方法即可。

### Q: 支持同步方法吗？
**A**: 是的，工具方法可以是同步或异步的：
```rust
#[tool(description = "同步方法")]
pub fn sync_tool(&self) -> Result<CallToolResult, ErrorData> { }

#[tool(description = "异步方法")]
pub async fn async_tool(&self) -> Result<CallToolResult, ErrorData> { }
```

### Q: 如何返回结构化数据？
**A**: 使用 `CallToolResult::structured`：
```rust
use serde_json::json;

let data = json!({
    "ip": "203.0.113.42",
    "city": "Beijing"
});
Ok(CallToolResult::structured(data))
```

---

**最后更新**: 2025-12-19
