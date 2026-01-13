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

1. 修改 GitHub 用户名

编辑 scripts/install.js:39-40，将 liushoukai 改为你的 GitHub 用户名：
const GITHUB_USER = 'your-github-username';
const GITHUB_REPO = 'rust-mcp-client';

2. 创建 npm 账号（如果没有）
   访问 https://www.npmjs.com/signup
3. 发布流程

# 1. 登录 npm
npm login

# 2. 创建 git tag（会触发 GitHub Actions 自动构建）
git add .
git commit -m "feat: Add npm wrapper"
git tag v0.1.0
git push origin main --tags

# 3. 等待 GitHub Actions 完成（约 10-15 分钟）
# 访问 https://github.com/你的用户名/rust-mcp-client/actions

# 4. 确认 Release 已创建
# 访问 https://github.com/你的用户名/rust-mcp-client/releases

# 5. 发布到 npm
npm publish --access public

后续版本发布

# 1. 同步更新两个文件的版本号
#    - package.json: "version": "0.1.1"
#    - Cargo.toml: version = "0.1.1"

# 2. 提交并打 tag
git add .
git commit -m "Release v0.1.1"
git tag v0.1.1
git push origin main --tags

# 3. 等待 Actions 完成后，发布到 npm
方案 1: 发布为公开包（推荐）

在发布命令中添加 --access public 参数：

npm publish --access public


npm publish --access public

发布成功后，用户就可以这样使用：

npx @liushoukai/rust-mcp-client

或者在 Cursor/Claude Desktop 配置中：
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



🎯 工作原理

用户配置 npx @liushoukai/rust-mcp-client
↓
npm 下载包（几 KB 的 JS 文件）
↓
执行 postinstall: scripts/install.js
↓
从 GitHub Release 下载对应平台的 Rust 二进制
↓
bin/index.js 启动 Rust 程序
↓
透传输入输出，完成 MCP 通信


📝 注意事项

1. 首次发布前，确保在 scripts/install.js 中修改了 GitHub 用户名
2. 确保 package.json 和 Cargo.toml 的版本号保持一致
3. npm 包名 @liushoukai/rust-mcp-client 可以改成你想要的名字（需要在 npm 上可用）
