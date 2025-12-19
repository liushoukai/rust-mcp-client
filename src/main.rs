use rmcp::transport::stdio;
use rmcp::ServiceExt;
use rust_mcp_client::server::IpInfoServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建 IP 信息服务
    let service = IpInfoServer::new();

    // 使用 stdio 传输启动 MCP 服务器
    let server = service.serve(stdio()).await?;

    // 等待服务器结束
    server.waiting().await?;

    Ok(())
}
