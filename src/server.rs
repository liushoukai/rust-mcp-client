use crate::mcp::*;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct McpServer;

impl McpServer {
    pub fn new() -> Self {
        McpServer
    }

    // 启动MCP server，从stdin读取请求，向stdout写入响应
    pub async fn run(&self) -> std::io::Result<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        eprintln!("MCP Server started, waiting for requests...");

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;

            // 如果没有读取到数据，说明stdin已关闭
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            eprintln!("Received request: {}", trimmed);

            // 解析JSON-RPC请求
            match serde_json::from_str::<JsonRpcRequest>(trimmed) {
                Ok(request) => {
                    let response = self.handle_request(request).await;
                    let response_json = serde_json::to_string(&response)?;
                    eprintln!("Sending response: {}", response_json);

                    // 写入响应到stdout
                    stdout.write_all(response_json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
                Err(e) => {
                    eprintln!("Failed to parse request: {}", e);
                    // 返回解析错误响应
                    let error_response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32700,
                            message: "Parse error".to_string(),
                            data: Some(json!({"details": e.to_string()})),
                        }),
                    };
                    let response_json = serde_json::to_string(&error_response)?;
                    stdout.write_all(response_json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
            }
        }

        eprintln!("MCP Server stopped");
        Ok(())
    }

    // 处理JSON-RPC请求，根据method路由到不同的处理函数
    async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let method = request.method.as_str();
        let id = request.id.clone();

        match method {
            "initialize" => self.handle_initialize(id, request.params),
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, request.params).await,
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            },
        }
    }

    // 处理initialize请求
    fn handle_initialize(
        &self,
        id: Option<Value>,
        _params: Option<Value>,
    ) -> JsonRpcResponse {
        let result = InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {}),
            },
            server_info: Implementation {
                name: "ip-info-server".to_string(),
                version: "0.1.0".to_string(),
            },
        };

        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::to_value(result).unwrap()),
            error: None,
        }
    }

    // 处理tools/list请求
    fn handle_tools_list(&self, id: Option<Value>) -> JsonRpcResponse {
        let tools = vec![Tool {
            name: "get_ip_info".to_string(),
            description: "获取当前机器的公网IP信息，包括IP地址、地理位置、ISP等".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }];

        let result = ToolsListResult { tools };

        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::to_value(result).unwrap()),
            error: None,
        }
    }

    // 处理tools/call请求
    async fn handle_tools_call(
        &self,
        id: Option<Value>,
        params: Option<Value>,
    ) -> JsonRpcResponse {
        // 解析参数
        let params: ToolsCallParams = match params {
            Some(p) => match serde_json::from_value(p) {
                Ok(params) => params,
                Err(e) => {
                    return JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: Some(json!({"details": e.to_string()})),
                        }),
                    }
                }
            },
            None => {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing params".to_string(),
                        data: None,
                    }),
                }
            }
        };

        // 根据工具名称执行相应的操作
        match params.name.as_str() {
            "get_ip_info" => {
                match crate::fetch_ip_info().await {
                    Ok(ip_info) => {
                        let result = ToolsCallResult {
                            content: vec![ToolContent {
                                content_type: "text".to_string(),
                                text: ip_info.to_string(),
                            }],
                            is_error: None,
                        };

                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: Some(serde_json::to_value(result).unwrap()),
                            error: None,
                        }
                    }
                    Err(e) => {
                        let result = ToolsCallResult {
                            content: vec![ToolContent {
                                content_type: "text".to_string(),
                                text: format!("获取IP信息失败: {}", e),
                            }],
                            is_error: Some(true),
                        };

                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: Some(serde_json::to_value(result).unwrap()),
                            error: None,
                        }
                    }
                }
            }
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: format!("Unknown tool: {}", params.name),
                    data: None,
                }),
            },
        }
    }
}
