# 日志配置指南

## 概述

rust-mcp-client 使用 `tracing` 库来输出详细的运行日志。通过设置 `RUST_LOG` 环境变量,你可以控制日志的详细程度。

## 快速参考

| 使用场景 | 日志位置 | 如何查看 |
|---------|---------|---------|
| **Claude Desktop** | 开发者工具 Console | `Cmd+Option+I` → Console 标签页 |
| **Cursor** | 开发者工具 Console | `Ctrl+Shift+I` → Console 标签页 |
| **命令行测试** | stderr (终端) | 直接在终端看到输出 |
| **macOS 系统日志** | Console.app | `log stream --predicate 'process == "Claude"'` |

**重要**:
- ✅ 日志输出到 **stderr** (不影响 MCP JSON-RPC 通信)
- ✅ 需要版本 **>= 0.1.2** 才包含日志功能
- ✅ 必须在配置中添加 `env: {"RUST_LOG": "debug"}`

## 日志级别

从最简单到最详细:

- `error` - 仅显示错误信息
- `warn` - 显示警告和错误
- `info` - 显示一般信息 (默认级别)
- `debug` - 显示调试信息
- `trace` - 显示最详细的跟踪信息

## 在 Claude Desktop 中启用详细日志

编辑配置文件 `~/Library/Application Support/Claude/claude_desktop_config.json`:

### 方式一: 使用 npx (推荐)

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

### 方式二: 使用全局安装的版本

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "rust-mcp-client",
      "env": {
        "RUST_LOG": "debug"
      }
    }
  }
}
```

### 方式三: 使用编译后的二进制文件

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "/path/to/rust-mcp-client/target/release/rust-mcp-client",
      "env": {
        "RUST_LOG": "debug"
      }
    }
  }
}
```

## 在 Cursor 中启用详细日志

配置方法与 Claude Desktop 相同,编辑 Cursor 的 MCP 配置文件。

## 高级日志配置

### 只显示特定模块的日志

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"],
      "env": {
        "RUST_LOG": "rust_mcp_client=debug"
      }
    }
  }
}
```

### 组合多个日志级别

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"],
      "env": {
        "RUST_LOG": "rust_mcp_client=debug,rmcp=info"
      }
    }
  }
}
```

### 最详细的日志 (用于调试问题)

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"],
      "env": {
        "RUST_LOG": "trace"
      }
    }
  }
}
```

## 查看日志

### 日志输出位置说明

Rust 程序将日志输出到 **stderr**（标准错误流），这样不会干扰 MCP 协议的 JSON-RPC 通信（通过 stdout）。

### Claude Desktop / Cursor

**这是查看日志的主要方式** ⭐

日志会被应用捕获并显示在开发者工具中:

1. 打开 Claude Desktop 或 Cursor
2. 按 `Cmd+Option+I` (macOS) 或 `Ctrl+Shift+I` (Windows/Linux) 打开开发者工具
3. 切换到 **"Console"** 标签页
4. 在配置中设置 `RUST_LOG` 环境变量后，重启应用
5. 使用 MCP 工具时，日志会实时显示在 Console 中

**日志示例:**
```
[INFO rust_mcp_client:21] 正在启动 MCP IP 信息服务器...
[DEBUG rust_mcp_client:26] IP 信息服务创建完成
[INFO rust_mcp_client:31] MCP 服务器已启动,等待请求...
[INFO rust_mcp_client::server:26] 收到获取 IP 信息请求
[DEBUG rust_mcp_client::server:27] 正在调用 IP 信息 API...
```

### macOS 系统日志

Claude Desktop 的日志也可能出现在 macOS 系统日志中:

```bash
# 实时查看 Claude Desktop 的日志
log stream --predicate 'process == "Claude"' --level debug

# 查看最近的日志
log show --predicate 'process == "Claude"' --last 1h
```

### 命令行测试

你也可以直接在命令行测试日志输出:

```bash
# Info 级别 (默认)
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/rust-mcp-client

# Debug 级别
RUST_LOG=debug echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_ip_info","arguments":{}}}' | ./target/release/rust-mcp-client

# Trace 级别 (最详细)
RUST_LOG=trace echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_ip_info","arguments":{}}}' | ./target/release/rust-mcp-client
```

## 日志输出示例

### Info 级别 (默认)

```
2026-01-13T10:42:48.250378Z  INFO rust_mcp_client:21: 正在启动 MCP IP 信息服务器...
2026-01-13T10:42:48.251234Z  INFO rust_mcp_client:31: MCP 服务器已启动,等待请求...
```

### Debug 级别

```
2026-01-13T10:42:48.250378Z  INFO rust_mcp_client:21: 正在启动 MCP IP 信息服务器...
2026-01-13T10:42:48.250912Z DEBUG rust_mcp_client:26: IP 信息服务创建完成
2026-01-13T10:42:48.251234Z  INFO rust_mcp_client:31: MCP 服务器已启动,等待请求...
2026-01-13T10:42:48.252145Z  INFO rust_mcp_client::server:26: 收到获取 IP 信息请求
2026-01-13T10:42:48.252389Z DEBUG rust_mcp_client::server:27: 正在调用 IP 信息 API...
2026-01-13T10:42:48.252567Z DEBUG rust_mcp_client:44: 正在从 https://ipinfo.io/json 获取 IP 信息
2026-01-13T10:42:48.456789Z DEBUG rust_mcp_client:47: 收到响应,状态码: 200 OK
2026-01-13T10:42:48.457123Z DEBUG rust_mcp_client:50: 成功解析 IP 信息: IpInfo { ip: "1.2.3.4", ... }
2026-01-13T10:42:48.457456Z  INFO rust_mcp_client::server:31: 成功获取 IP 信息: ...
```

## 故障排查

如果日志没有显示:

1. **确认版本**: npm 上发布的版本需要 >= 0.1.2 才包含日志功能
   ```bash
   # 查看已安装的版本
   npm list -g @liushoukai/rust-mcp-client

   # 或者查看 npx 缓存的版本
   npx @liushoukai/rust-mcp-client --version

   # 清除 npx 缓存并重新下载
   rm -rf ~/.npm/_npx
   ```

2. **确认环境变量已正确设置**: 检查配置文件中的 `env` 字段，确保有:
   ```json
   "env": {
     "RUST_LOG": "debug"
   }
   ```

3. **重启 Claude Desktop**: 配置更改后需要 **完全退出** 并重新启动应用
   - macOS: `Cmd+Q` 完全退出，然后重新打开
   - 不要只是关闭窗口，要完全退出应用

4. **检查开发者工具**: 确保已打开 Claude Desktop 的开发者工具 Console 标签页
   - 按 `Cmd+Option+I` (macOS) 或 `Ctrl+Shift+I` (Windows/Linux)
   - 必须 **先打开开发者工具**，然后再触发 MCP 请求

5. **验证二进制文件**: 如果使用本地编译的版本，确保使用的是最新编译的版本 (包含日志功能)
   ```bash
   cargo build --release
   ```

## 性能考虑

- `info` 级别对性能影响很小,可以在生产环境使用
- `debug` 级别会产生较多输出,建议仅在开发或调试时使用
- `trace` 级别会产生大量输出,仅在需要深入调试问题时使用

## 禁用日志

如果想完全禁用日志输出,可以设置:

```json
{
  "mcpServers": {
    "ip-info": {
      "command": "npx",
      "args": ["-y", "@liushoukai/rust-mcp-client"],
      "env": {
        "RUST_LOG": "off"
      }
    }
  }
}
```

或者干脆不设置 `env` 字段 (将使用默认的 `info` 级别)。
