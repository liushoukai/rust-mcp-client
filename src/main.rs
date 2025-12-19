use rust_mcp_client::server::McpServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = McpServer::new();
    server.run().await
}
