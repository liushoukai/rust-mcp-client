# Rust MCP Server - IP 信息查询服务

一个使用 Rust 和官方 rmcp SDK 实现的 Model Context Protocol (MCP) 服务器，提供公网 IP 信息查询功能。

## 功能特性

- ✅ 获取公网 IP 地址
- ✅ 查询 IP 地理位置（城市、地区、国家）
- ✅ 获取 ISP 信息
- ✅ 显示时区信息
- ✅ 提供经纬度坐标

## 技术栈

- **Rust** - 高性能系统编程语言
- **rmcp 0.9.0** - 官方 MCP SDK
- **tokio** - 异步运行时
- **reqwest** - HTTP 客户端
- **serde** - 序列化框架

## 项目结构

```
rust-mcp-client/
├── src/
│   ├── main.rs      # 服务器启动入口
│   ├── server.rs    # MCP 服务器实现
│   └── lib.rs       # IP 信息获取逻辑
├── docs/
│   └── MCP_PROTOCOL.md  # MCP 协议详解
├── Cargo.toml       # 项目依赖配置
└── README.md        # 本文档
```

## 快速开始

### 方式一：使用 npm（推荐）⭐

无需安装 Rust，自动下载适合你系统的二进制文件。

#### 直接使用（npx）

```bash
npx @liushoukai/rust-mcp-client
```

#### 全局安装

```bash
npm install -g @liushoukai/rust-mcp-client
rust-mcp-client
```

### 方式二：从源代码编译

需要安装 Rust 工具链。

```bash
# Debug 版本
cargo build

# Release 版本（推荐用于生产环境）
cargo build --release
```

编译后的可执行文件位于：
- Debug: `target/debug/rust-mcp-client`
- Release: `target/release/rust-mcp-client`

### 方式三：使用 cargo install

```bash
# 从 crates.io 安装（如果已发布）
cargo install rust-mcp-client

# 从本地源码安装
cargo install --path .
```

### 2. 测试服务器

手动测试服务器是否正常工作：

```bash
# 测试 initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/rust-mcp-client

# 测试 tools/list
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | ./target/release/rust-mcp-client

# 测试 tools/call
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_ip_info","arguments":{}}}' | ./target/release/rust-mcp-client
```

### 3. 配置 MCP 客户端

#### Claude Desktop 配置

**方式一：使用 npx（推荐，无需安装）**

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

**方式二：使用全局安装的版本**

先安装：
```bash
npm install -g @liushoukai/rust-mcp-client
```

然后配置：
```json
{
  "mcpServers": {
    "ip-info": {
      "command": "rust-mcp-client"
    }
  }
}
```

**方式三：使用编译后的二进制文件**

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "/path/to/rust-mcp-client/target/release/rust-mcp-client"
    }
  }
}
```

#### Cursor 配置

编辑配置文件：
- macOS: `~/.cursor/mcp_settings.json` 或 `~/Library/Application Support/Cursor/User/globalStorage/mcp_settings.json`
- Windows: `%APPDATA%\Cursor\User\globalStorage\mcp_settings.json`
- Linux: `~/.config/Cursor/User/globalStorage/mcp_settings.json`

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

### 4. 重启 Claude Desktop

配置完成后，完全退出并重新启动 Claude Desktop。

### 5. 使用工具

在 Claude Desktop 中直接询问：

```
请帮我获取当前机器的公网 IP 信息
```

或者：

```
使用 get_ip_info 工具查看我的 IP 地址
```

## 代码示例

### 服务器实现

```rust
use rmcp::*;
use rmcp::model::{CallToolResult, ServerInfo, ServerCapabilities, Content};
use rmcp::handler::server::router::tool::ToolRouter;

#[derive(Clone)]
pub struct IpInfoServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl IpInfoServer {
    #[tool(description = "获取当前机器的公网IP信息")]
    pub async fn get_ip_info(&self) -> Result<CallToolResult, ErrorData> {
        match crate::fetch_ip_info().await {
            Ok(ip_info) => {
                let content = Content::text(ip_info.to_string());
                Ok(CallToolResult::success(vec![content]))
            }
            Err(e) => {
                Err(ErrorData::internal_error(
                    format!("获取IP信息失败: {}", e),
                    None
                ))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for IpInfoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("提供公网 IP 信息查询服务".into()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
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
    let service = IpInfoServer::new();
    let server = service.serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
```

## 代码优化历程

### 重构前 vs 重构后

| 指标 | 重构前（手动实现） | 重构后（使用 SDK） | 改进 |
|------|------------------|-------------------|------|
| 代码行数 | ~360 行 | ~100 行 | **-72%** |
| server.rs | 231 行 | 41 行 | **-82%** |
| main.rs | 8 行 | 16 行 | +100%（更清晰） |
| mcp.rs | 118 行 | 已删除 | **-100%** |
| 样板代码 | 大量手动定义 | SDK 自动生成 | **显著减少** |
| 可维护性 | 需手动同步协议 | SDK 自动更新 | **大幅提升** |

### 主要改进

1. **删除所有手动协议实现** - 不再需要手动定义 JSON-RPC 结构
2. **使用宏简化工具定义** - `#[tool]` 自动生成路由和 Schema
3. **自动处理协议细节** - SDK 处理所有 MCP 协议方法
4. **更好的类型安全** - 利用 Rust 类型系统和 SDK 类型
5. **标准化实现** - 与官方规范保持一致

## 依赖说明

```toml
[dependencies]
rmcp = { version = "0.9.0", features = ["server", "transport-io"] }
reqwest = { version = "0.12.26", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
serde = { version = "1.0.228", features = ["derive"] }
schemars = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
```

## 调试

### 查看 Claude Desktop 日志

- **macOS**: `~/Library/Logs/Claude/`
- **Windows**: `%APPDATA%\Claude\logs\`
- **Linux**: `~/.config/Claude/logs/`

### 使用 MCP Inspector

```bash
npx @modelcontextprotocol/inspector ./target/release/rust-mcp-client
```

这会启动一个 Web 界面，可以交互式测试 MCP 服务器。

## 发布流程

### 发布到 npm

1. **创建 GitHub Release（自动构建二进制文件）**

```bash
# 更新版本号（同步 package.json 和 Cargo.toml）
# package.json: "version": "0.1.1"
# Cargo.toml: version = "0.1.1"

# 提交并打 tag
git add .
git commit -m "Release v0.1.1"
git tag v0.1.1
git push origin main --tags
```

2. **等待 GitHub Actions 完成构建**

查看 Actions 页面，确保所有平台的二进制文件都已构建完成并上传到 Release。

3. **发布到 npm**

```bash
# 登录 npm（首次需要）
npm login

# 发布（确保 package.json 版本号与 git tag 一致）
npm publish --access public
```

### 版本发布检查清单

- [ ] 更新 `package.json` 中的版本号
- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 更新 CHANGELOG（可选）
- [ ] 提交代码并打 tag
- [ ] 推送到 GitHub 触发 Actions
- [ ] 确认 GitHub Release 包含所有平台二进制文件
- [ ] 运行 `npm publish` 发布到 npm
- [ ] 测试安装：`npx @liushoukai/rust-mcp-client@latest`

## 扩展开发

### 添加新工具

在 `IpInfoServer` 的 `#[tool_router]` impl 块中添加新方法：

```rust
#[tool_router]
impl IpInfoServer {
    #[tool(description = "获取IP信息")]
    pub async fn get_ip_info(&self) -> Result<CallToolResult, ErrorData> {
        // 现有实现
    }

    // 新增工具
    #[tool(description = "检查IP是否在某个范围内")]
    pub async fn check_ip_range(
        &self,
        #[tool(param)]
        #[schemars(description = "CIDR 格式的 IP 范围")]
        cidr: String
    ) -> Result<CallToolResult, ErrorData> {
        // 实现逻辑
        todo!()
    }
}
```

### 添加资源支持

实现 `resources/list` 和 `resources/read`：

```rust
impl ServerHandler for IpInfoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()  // 启用资源
                .build(),
            ..Default::default()
        }
    }

    // 实现资源方法
    async fn list_resources(&self, _params: ListResourcesParams)
        -> Result<ListResourcesResult, ErrorData>
    {
        // 返回资源列表
        todo!()
    }
}
```

## 常见问题

### Q: 编译报错怎么办？
**A**: 确保使用 Rust 1.70+ 版本：
```bash
rustc --version
rustup update
```

### Q: Claude Desktop 找不到工具？
**A**:
1. 检查配置文件路径是否正确
2. 确保可执行文件有执行权限
3. 重启 Claude Desktop
4. 查看日志文件

### Q: 如何返回结构化数据？
**A**: 使用 `CallToolResult::structured`：
```rust
use serde_json::json;

let data = json!({
    "ip": "203.0.113.42",
    "location": {
        "city": "Beijing",
        "country": "China"
    }
});
Ok(CallToolResult::structured(data))
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 相关资源

- [MCP 协议详解](docs/MCP_PROTOCOL.md)
- [MCP 官方规范](https://spec.modelcontextprotocol.io/)
- [rmcp 文档](https://docs.rs/rmcp/)
- [官方 Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)

---

**项目作者**: liushoukai
**最后更新**: 2025-12-19
