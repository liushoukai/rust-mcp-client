# Claude Desktop 配置和调试指南

## 第一步：配置 MCP 服务器

编辑配置文件：`~/Library/Application Support/Claude/claude_desktop_config.json`

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

## 第二步：重启 Claude Desktop

**重要**: 必须完全退出并重新启动

- macOS: 按 `Cmd+Q` 完全退出，然后重新打开
- **不要**只是关闭窗口

## 第三步：打开开发者工具

1. 启动 Claude Desktop
2. 按 `Cmd+Option+I` (macOS) 打开开发者工具
3. 切换到 **"Console"** 标签页
4. (可选) 在 Console 右上角的过滤框中输入 `rust_mcp_client` 只显示相关日志

## 第四步：使用 MCP 工具

在 Claude Desktop 的聊天框中输入:

```
请帮我获取当前机器的公网 IP 信息
```

## 第五步：查看日志

在开发者工具的 Console 中，你应该能看到类似这样的日志：

```
[2026-01-13T10:42:48.250Z] INFO rust_mcp_client: 正在启动 MCP IP 信息服务器...
[2026-01-13T10:42:48.250Z] DEBUG rust_mcp_client: IP 信息服务创建完成
[2026-01-13T10:42:48.251Z] INFO rust_mcp_client: MCP 服务器已启动,等待请求...
[2026-01-13T10:42:48.252Z] INFO rust_mcp_client::server: 收到获取 IP 信息请求
[2026-01-13T10:42:48.252Z] DEBUG rust_mcp_client::server: 正在调用 IP 信息 API...
[2026-01-13T10:42:48.252Z] DEBUG rust_mcp_client: 正在从 https://ipinfo.io/json 获取 IP 信息
[2026-01-13T10:42:48.456Z] DEBUG rust_mcp_client: 收到响应,状态码: 200 OK
[2026-01-13T10:42:48.457Z] DEBUG rust_mcp_client: 成功解析 IP 信息
[2026-01-13T10:42:48.457Z] INFO rust_mcp_client::server: 成功获取 IP 信息
```

## 故障排查

### 问题：看不到日志

**检查清单:**

1. ✅ 配置文件中有 `"env": {"RUST_LOG": "debug"}` 吗？
2. ✅ 是否完全退出并重启了 Claude Desktop？(`Cmd+Q`)
3. ✅ 开发者工具的 Console 标签页是否已打开？
4. ✅ npm 包版本是否 >= 0.1.2？

**验证版本:**
```bash
# 清除 npx 缓存，强制重新下载最新版本
rm -rf ~/.npm/_npx

# 或者全局安装特定版本
npm install -g @liushoukai/rust-mcp-client@latest
```

### 问题：MCP 工具没有出现

**检查 MCP 状态:**

1. 在 Claude Desktop 窗口右下角，查看是否有 MCP 图标
2. 点击 MCP 图标，查看 `ip-info` 服务器的状态
3. 如果显示错误，查看错误信息

**常见错误:**

- `ENOENT: no such file or directory` - npx 找不到命令，检查网络连接
- `permission denied` - 二进制文件权限问题，重新安装

### 问题：日志太多或太少

**调整日志级别:**

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"],
      "env": {
        "RUST_LOG": "info"   // 改为 "info" 减少日志
        // 或
        "RUST_LOG": "trace"  // 改为 "trace" 增加日志
      }
    }
  }
}
```

## 日志级别说明

| 级别 | 适用场景 | 输出量 |
|------|---------|--------|
| `error` | 生产环境，只看错误 | 极少 |
| `warn` | 生产环境，看警告和错误 | 少 |
| `info` | 默认级别，看一般信息 | 中等 |
| `debug` | 开发调试，详细信息 ⭐ | 多 |
| `trace` | 深度调试，最详细 | 非常多 |

**推荐配置:**
- 日常使用: `info`
- 调试问题: `debug`
- 深度调试: `trace`

## 其他有用的命令

### 查看 Claude Desktop 日志文件

```bash
# macOS 系统日志
log stream --predicate 'process == "Claude"' --level debug

# 查看最近 1 小时的日志
log show --predicate 'process == "Claude"' --last 1h
```

### 测试 MCP 服务器 (不通过 Claude Desktop)

```bash
# 使用 echo 发送 JSON-RPC 请求
RUST_LOG=debug npx -y @liushoukai/rust-mcp-client << 'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized"}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_ip_info","arguments":{}}}
EOF
```

## 参考资源

- [完整日志文档](./LOGGING.md)
- [MCP 协议文档](./docs/MCP_PROTOCOL.md)
- [项目 README](./README.md)
