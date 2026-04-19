extern crate github_recon;
use github_recon::api::{GitHubClient, SearchQuery};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResult {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: serde_json::Value,
    pub client_info: serde_json::Value,
}

fn send_response(response: ToolResult) -> Result<()> {
    let json = serde_json::to_string(&response)?;
    println!("{}", json);
    std::io::stdout().flush()?;
    Ok(())
}

fn handle_initialize(_params: serde_json::Value) -> Result<serde_json::Value> {
    Ok(serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": "github-recon",
            "version": "0.1.0"
        }
    }))
}

fn handle_list_tools() -> Result<serde_json::Value> {
    Ok(serde_json::json!({
        "tools": [
            {
                "name": "search_repositories",
                "description": "Search GitHub repositories and return results as CSV",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query string"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Maximum number of results (default: 100)",
                            "default": 100
                        },
                        "sort": {
                            "type": "string",
                            "enum": ["stars", "forks", "updated"],
                            "description": "Sort field"
                        },
                        "order": {
                            "type": "string",
                            "enum": ["asc", "desc"],
                            "description": "Sort order"
                        }
                    },
                    "required": ["query"]
                }
            }
        ]
    }))
}

async fn handle_search_repositories(args: serde_json::Value) -> Result<String> {
    let query = args["query"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;
    
    let limit = args["limit"]
        .as_u64()
        .unwrap_or(100) as usize;
    
    let sort = args["sort"]
        .as_str()
        .unwrap_or("stars");
    
    let order = args["order"]
        .as_str()
        .unwrap_or("desc");

    let client = GitHubClient::new();
    let search_query = SearchQuery::new(query)
        .with_pagination(1, limit.min(100))
        .with_sort(sort, order);

    let repos = client.search_all_repositories(&search_query, Some(limit)).await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["url", "name", "description", "stars", "forks", "last_updated", "language"])?;
    
    for repo in repos {
        wtr.serialize(serde_json::json!({
            "url": repo.html_url,
            "name": repo.full_name,
            "description": repo.description.unwrap_or_default(),
            "stars": repo.stargazers_count,
            "forks": repo.forks_count,
            "last_updated": repo.updated_at,
            "language": repo.language.unwrap_or_default(),
        }))?;
    }
    
    let data = String::from_utf8(wtr.into_inner()?)?;
    Ok(data)
}

fn handle_method_call(method: &str, params: Option<serde_json::Value>, id: serde_json::Value) -> Result<()> {
    let params = params.unwrap_or(serde_json::Value::Null);
    
    let result = match method {
        "initialize" => {
            let result = serde_json::to_value(handle_initialize(params)?)?;
            ToolResult {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(result),
                error: None,
            }
        }
        "tools/list" => {
            let result = serde_json::to_value(handle_list_tools()?)?;
            ToolResult {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(result),
                error: None,
            }
        }
        "tools/call" => {
            let tool_name = params["name"].as_str().unwrap_or("");
            let arguments = &params["arguments"];
            
            if tool_name == "search_repositories" {
                let rt = tokio::runtime::Runtime::new()?;
                let csv_data = rt.block_on(handle_search_repositories(arguments.clone()))?;
                
                ToolResult {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!({
                        "content": [{
                            "type": "text",
                            "text": csv_data
                        }]
                    })),
                    error: None,
                }
            } else {
                ToolResult {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(serde_json::json!({
                        "code": -32601,
                        "message": format!("Unknown tool: {}", tool_name)
                    })),
                }
            }
        }
        _ => {
            ToolResult {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(serde_json::json!({
                    "code": -32601,
                    "message": format!("Method not found: {}", method)
                })),
            }
        }
    };
    
    send_response(result)
}

pub fn run_mcp_server() -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    
    loop {
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        
        let call: ToolCall = match serde_json::from_str(&input) {
            Ok(c) => c,
            Err(_) => continue,
        };
        
        rt.block_on(async {
            handle_method_call(&call.method, call.params, call.id).ok();
        });
    }
    
    Ok(())
}

fn main() {
    if let Err(e) = run_mcp_server() {
        eprintln!("MCP server error: {}", e);
        std::process::exit(1);
    }
}
