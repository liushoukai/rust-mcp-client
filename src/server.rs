use rmcp::*;
use rmcp::model::{CallToolResult, ServerInfo, ServerCapabilities, Content};
use rmcp::handler::server::router::tool::ToolRouter;

#[derive(Clone)]
pub struct IpInfoServer {
    // tool_router 字段是必需的，#[tool_handler] 宏会使用它
    tool_router: ToolRouter<Self>,
}

impl IpInfoServer {
    pub fn new() -> Self {
        IpInfoServer {
            // 调用由 #[tool_router] 宏自动生成的方法
            tool_router: Self::tool_router(),
        }
    }
}

// 使用 tool_router 宏标注工具方法所在的 impl 块
#[tool_router]
impl IpInfoServer {
    /// 获取当前机器的公网IP信息，包括IP地址、地理位置、ISP等
    #[tool(description = "获取当前机器的公网IP信息，包括IP地址、地理位置、ISP等")]
    pub async fn get_ip_info(&self) -> Result<CallToolResult, ErrorData> {
        tracing::info!("收到获取 IP 信息请求");
        tracing::debug!("正在调用 IP 信息 API...");

        match crate::fetch_ip_info().await {
            Ok(ip_info) => {
                tracing::info!("成功获取 IP 信息: {}", ip_info);
                let content = Content::text(ip_info.to_string());
                Ok(CallToolResult::success(vec![content]))
            }
            Err(e) => {
                tracing::error!("获取 IP 信息失败: {}", e);
                Err(ErrorData::internal_error(format!("获取IP信息失败: {}", e), None))
            }
        }
    }
}

// 使用 tool_handler 宏自动实现工具请求处理
#[tool_handler]
impl ServerHandler for IpInfoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("提供公网 IP 信息查询服务，可以获取 IP 地址、地理位置、ISP 等信息".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
