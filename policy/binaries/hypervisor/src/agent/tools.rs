use serde_json::json;
use std::fs;
use tracing::debug;

use super::policy_registry::PolicyRegistry;
use super::quote_utils::verify_compliance_quote_dummy;
use super::types::{ComplianceQuote, Tool, ToolCall, ToolResult};

// =============================================================================
// T1: PriceFeedTool - Policy: L1
// =============================================================================

/// T1: Price feed tool for cryptocurrency prices
pub struct PriceFeedTool {
    data: serde_json::Value,
}

impl PriceFeedTool {
    pub fn new() -> Result<Self, String> {
        let data_path = "binaries/hypervisor/data/price_feed.json";
        let data_str = fs::read_to_string(data_path)
            .map_err(|e| format!("Failed to read price feed data: {}", e))?;
        let data: serde_json::Value = serde_json::from_str(&data_str)
            .map_err(|e| format!("Failed to parse price feed data: {}", e))?;
        Ok(Self { data })
    }
}

impl Tool for PriceFeedTool {
    fn name(&self) -> &str {
        "PriceFeedTool"
    }

    fn description(&self) -> &str {
        "Get current cryptocurrency prices in USD. Returns real-time price data with timestamp."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "symbol": {
                    "type": "string",
                    "description": "The cryptocurrency symbol (e.g., BTC, ETH, SOL)"
                }
            },
            "required": ["symbol"]
        })
    }

    fn execute(&self, arguments: &str, compliance_quote: Option<&ComplianceQuote>) -> Result<String, String> {
        // Verify compliance quote (dummy verification)
        if let Some(quote) = compliance_quote {
            let verified = verify_compliance_quote_dummy(quote, self.name())
                .map_err(|e| format!("Quote verification error: {}", e))?;
            
            if !verified {
                return Err("Compliance quote verification failed".to_string());
            }
            
            if !quote.compliant {
                return Err("Tool use was rejected by compliance policy".to_string());
            }
            
            debug!("Compliance quote verified for {}", self.name());
        }
        
        let args: serde_json::Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Invalid arguments: {}", e))?;

        let symbol = args["symbol"]
            .as_str()
            .ok_or("Missing symbol parameter")?
            .to_uppercase();

        // Load price data from JSON
        let prices = self.data["prices"]
            .as_array()
            .ok_or("Invalid price data format")?;

        let price_data = prices
            .iter()
            .find(|p| p["symbol"].as_str() == Some(&symbol))
            .ok_or_else(|| format!("Unknown cryptocurrency: {}", symbol))?;

        Ok(json!({
            "tool": "PriceFeedTool",
            "symbol": symbol,
            "price_usd": price_data["price_usd"],
            "market_cap": price_data["market_cap"],
            "24h_volume": price_data["24h_volume"],
            "24h_change_pct": price_data["24h_change_pct"],
            "last_updated": price_data["last_updated"],
            "source": "Market Data Feed"
        })
        .to_string())
    }

    fn policy_ids(&self) -> Vec<String> {
        vec!["L1".to_string()]
    }

    fn policy_info(&self) -> Vec<super::policy_registry::PolicyInfo> {
        PolicyRegistry::default_crypto_policy().get_policy_info_for_tool(self.name())
    }
}

// =============================================================================
// T2: OnChainHistoryTool - Policy: L1, L2, L3
// =============================================================================

/// T2: On-chain transaction history tool
pub struct OnChainHistoryTool {
    data: serde_json::Value,
}

impl OnChainHistoryTool {
    pub fn new() -> Result<Self, String> {
        let data_path = "binaries/hypervisor/data/onchain_history.json";
        let data_str = fs::read_to_string(data_path)
            .map_err(|e| format!("Failed to read on-chain history data: {}", e))?;
        let data: serde_json::Value = serde_json::from_str(&data_str)
            .map_err(|e| format!("Failed to parse on-chain history data: {}", e))?;
        Ok(Self { data })
    }
}

impl Tool for OnChainHistoryTool {
    fn name(&self) -> &str {
        "OnChainHistoryTool"
    }

    fn description(&self) -> &str {
        "Get on-chain transaction history for a wallet address. Returns individual transaction records."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "address": {
                    "type": "string",
                    "description": "The wallet address to query (optional - if not provided, returns all addresses)"
                },
                "blockchain": {
                    "type": "string",
                    "description": "The blockchain network (e.g., ethereum, solana, bitcoin)"
                }
            },
            "required": ["blockchain"]
        })
    }

    fn execute(&self, arguments: &str, compliance_quote: Option<&ComplianceQuote>) -> Result<String, String> {
        // Verify compliance quote (dummy verification)
        if let Some(quote) = compliance_quote {
            let verified = verify_compliance_quote_dummy(quote, self.name())
                .map_err(|e| format!("Quote verification error: {}", e))?;
            
            if !verified {
                return Err("Compliance quote verification failed".to_string());
            }
            
            if !quote.compliant {
                return Err("Tool use was rejected by compliance policy".to_string());
            }
            
            debug!("Compliance quote verified for {}", self.name());
        }
        
        let args: serde_json::Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Invalid arguments: {}", e))?;

        let address_opt = args["address"].as_str();
        let blockchain = args["blockchain"]
            .as_str()
            .ok_or("Missing blockchain parameter")?
            .to_lowercase();

        // Load transaction history from JSON - returns individual records
        let chain_data = self.data[&blockchain]
            .as_object()
            .ok_or_else(|| format!("Unsupported blockchain: {}", blockchain))?;

        if let Some(address) = address_opt {
            // Return data for specific address
            let transactions = chain_data
                .get(address)
                .ok_or_else(|| format!("No transaction history found for address: {}", address))?;

            Ok(json!({
                "tool": "OnChainHistoryTool",
                "address": address,
                "blockchain": blockchain,
                "transactions": transactions,
                "count": transactions.as_array().map(|a| a.len()).unwrap_or(0),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": "On-Chain Data Provider"
            })
            .to_string())
        } else {
            // Return data for all addresses
            Ok(json!({
                "tool": "OnChainHistoryTool",
                "blockchain": blockchain,
                "all_addresses": chain_data,
                "address_count": chain_data.len(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": "On-Chain Data Provider"
            })
            .to_string())
        }
    }

    fn policy_ids(&self) -> Vec<String> {
        vec!["L1".to_string(), "L2".to_string(), "L3".to_string()]
    }

    fn policy_info(&self) -> Vec<super::policy_registry::PolicyInfo> {
        PolicyRegistry::default_crypto_policy().get_policy_info_for_tool(self.name())
    }
}

// =============================================================================
// T3: SentimentTool - Policy: L1, L4
// =============================================================================

/// T3: Market sentiment analysis tool
pub struct SentimentTool {
    data: serde_json::Value,
}

impl SentimentTool {
    pub fn new() -> Result<Self, String> {
        let data_path = "binaries/hypervisor/data/sentiment.json";
        let data_str = fs::read_to_string(data_path)
            .map_err(|e| format!("Failed to read sentiment data: {}", e))?;
        let data: serde_json::Value = serde_json::from_str(&data_str)
            .map_err(|e| format!("Failed to parse sentiment data: {}", e))?;
        Ok(Self { data })
    }
}

impl Tool for SentimentTool {
    fn name(&self) -> &str {
        "SentimentTool"
    }

    fn description(&self) -> &str {
        "Analyze market sentiment for cryptocurrencies from social media and news sources. Returns aggregated sentiment scores."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "symbol": {
                    "type": "string",
                    "description": "The cryptocurrency symbol (e.g., BTC, ETH, SOL)"
                },
                "timeframe": {
                    "type": "string",
                    "description": "Time period for analysis (e.g., '24h', '7d', '30d')",
                    "default": "24h"
                }
            },
            "required": ["symbol"]
        })
    }

    fn execute(&self, arguments: &str, compliance_quote: Option<&ComplianceQuote>) -> Result<String, String> {
        // Verify compliance quote (dummy verification)
        if let Some(quote) = compliance_quote {
            let verified = verify_compliance_quote_dummy(quote, self.name())
                .map_err(|e| format!("Quote verification error: {}", e))?;
            
            if !verified {
                return Err("Compliance quote verification failed".to_string());
            }
            
            if !quote.compliant {
                return Err("Tool use was rejected by compliance policy".to_string());
            }
            
            debug!("Compliance quote verified for {}", self.name());
        }
        
        let args: serde_json::Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Invalid arguments: {}", e))?;

        let symbol = args["symbol"]
            .as_str()
            .ok_or("Missing symbol parameter")?
            .to_uppercase();
        let timeframe = args["timeframe"].as_str().unwrap_or("24h");

        // Load sentiment data from JSON
        let symbol_data = self.data[&symbol]
            .as_object()
            .ok_or_else(|| format!("Sentiment data not available for: {}", symbol))?;

        let timeframe_data = symbol_data
            .get(timeframe)
            .ok_or_else(|| format!("No data available for timeframe: {}", timeframe))?;

        // Calculate aggregate metrics from individual records
        let empty_vec = vec![];
        let records = timeframe_data.as_array().unwrap_or(&empty_vec);
        let total_mentions: u32 = records
            .iter()
            .filter_map(|r| r["mention_count"].as_u64())
            .map(|n| n as u32)
            .sum();
        let avg_score: f64 = if !records.is_empty() {
            records
                .iter()
                .filter_map(|r| r["score"].as_f64())
                .sum::<f64>()
                / records.len() as f64
        } else {
            0.0
        };

        let sentiment_label = if avg_score >= 0.6 {
            "Positive"
        } else if avg_score >= 0.4 {
            "Neutral"
        } else {
            "Negative"
        };

        Ok(json!({
            "tool": "SentimentTool",
            "symbol": symbol,
            "timeframe": timeframe,
            "sentiment_score": avg_score,
            "sentiment_label": sentiment_label,
            "mentions_count": total_mentions,
            "records": timeframe_data,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "source": "Social Media & News Analytics"
        })
        .to_string())
    }

    fn policy_ids(&self) -> Vec<String> {
        vec!["L1".to_string(), "L4".to_string()]
    }

    fn policy_info(&self) -> Vec<super::policy_registry::PolicyInfo> {
        PolicyRegistry::default_crypto_policy().get_policy_info_for_tool(self.name())
    }
}

// =============================================================================
// T4: PortfolioTool - Policy: L1, L2, L3, L4
// =============================================================================

/// T4: Portfolio analysis tool
pub struct PortfolioTool {
    data: serde_json::Value,
}

impl PortfolioTool {
    pub fn new() -> Result<Self, String> {
        let data_path = "binaries/hypervisor/data/portfolio.json";
        let data_str = fs::read_to_string(data_path)
            .map_err(|e| format!("Failed to read portfolio data: {}", e))?;
        let data: serde_json::Value = serde_json::from_str(&data_str)
            .map_err(|e| format!("Failed to parse portfolio data: {}", e))?;
        Ok(Self { data })
    }
}

impl Tool for PortfolioTool {
    fn name(&self) -> &str {
        "PortfolioTool"
    }

    fn description(&self) -> &str {
        "Analyze cryptocurrency portfolio performance and composition. Returns individual holdings with details."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "address": {
                    "type": "string",
                    "description": "The wallet address to analyze (optional - if not provided, returns all addresses)"
                },
                "blockchain": {
                    "type": "string",
                    "description": "The blockchain network (e.g., ethereum, solana)"
                }
            },
            "required": ["blockchain"]
        })
    }

    fn execute(&self, arguments: &str, compliance_quote: Option<&ComplianceQuote>) -> Result<String, String> {
        // Verify compliance quote (dummy verification)
        if let Some(quote) = compliance_quote {
            let verified = verify_compliance_quote_dummy(quote, self.name())
                .map_err(|e| format!("Quote verification error: {}", e))?;
            
            if !verified {
                return Err("Compliance quote verification failed".to_string());
            }
            
            if !quote.compliant {
                return Err("Tool use was rejected by compliance policy".to_string());
            }
            
            debug!("Compliance quote verified for {}", self.name());
        }
        
        let args: serde_json::Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Invalid arguments: {}", e))?;

        let address_opt = args["address"].as_str();
        let blockchain = args["blockchain"]
            .as_str()
            .ok_or("Missing blockchain parameter")?
            .to_lowercase();

        // Load portfolio data from JSON - returns individual holdings
        let chain_data = self.data[&blockchain]
            .as_object()
            .ok_or_else(|| format!("Unsupported blockchain: {}", blockchain))?;

        if let Some(address) = address_opt {
            // Return data for specific address
            let portfolio_data = chain_data
                .get(address)
                .ok_or_else(|| format!("No portfolio data found for address: {}", address))?;

            Ok(json!({
                "tool": "PortfolioTool",
                "address": address,
                "blockchain": blockchain,
                "holdings": portfolio_data["holdings"],
                "total_value_usd": portfolio_data["total_value_usd"],
                "num_tokens": portfolio_data["holdings"].as_array().map(|a| a.len()).unwrap_or(0),
                "last_updated": portfolio_data["last_updated"],
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": "Portfolio Analytics Provider"
            })
            .to_string())
        } else {
            // Return data for all addresses
            Ok(json!({
                "tool": "PortfolioTool",
                "blockchain": blockchain,
                "all_portfolios": chain_data,
                "address_count": chain_data.len(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": "Portfolio Analytics Provider"
            })
            .to_string())
        }
    }

    fn policy_ids(&self) -> Vec<String> {
        vec!["L1".to_string(), "L2".to_string(), "L3".to_string(), "L4".to_string()]
    }

    fn policy_info(&self) -> Vec<super::policy_registry::PolicyInfo> {
        PolicyRegistry::default_crypto_policy().get_policy_info_for_tool(self.name())
    }
}

// =============================================================================
// Tool Registry
// =============================================================================

/// Tool registry for managing available tools
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new tool registry with T1-T4 realistic crypto tools
    pub fn new_crypto_tools() -> Result<Self, String> {
        Ok(Self {
            tools: vec![
                Box::new(PriceFeedTool::new()?),
                Box::new(OnChainHistoryTool::new()?),
                Box::new(SentimentTool::new()?),
                Box::new(PortfolioTool::new()?),
            ],
        })
    }

    /// Get a tool by name
    pub fn get_tool(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.iter().find(|t| t.name() == name).map(|b| &**b)
    }

    /// Get all available tools
    pub fn all_tools(&self) -> &[Box<dyn Tool>] {
        &self.tools
    }

    /// Execute a tool call with compliance quote verification
    pub fn execute_tool_call(&self, call: &ToolCall) -> ToolResult {
        let result = self
            .get_tool(&call.tool_name)
            .ok_or_else(|| format!("Tool not found: {}", call.tool_name))
            .and_then(|tool| tool.execute(&call.arguments, call.compliance_quote.as_ref()));

        match result {
            Ok(data) => ToolResult {
                call_id: call.id,
                success: true,
                result: data,
                error: None,
                quote_verified: call.compliance_quote.is_some(), // Quote was present and verified
            },
            Err(e) => ToolResult {
                call_id: call.id,
                success: false,
                result: String::new(),
                error: Some(e),
                quote_verified: false,
            },
        }
    }

    /// Generate tool descriptions for LLM prompt
    pub fn generate_tool_descriptions(&self) -> String {
        let mut descriptions = String::from("Available tools:\n\n");

        for tool in &self.tools {
            let policy_info = tool.policy_info();
            let policy_str = policy_info
                .iter()
                .map(|p| format!("{} ({})", p.id, p.name))
                .collect::<Vec<_>>()
                .join(", ");

            descriptions.push_str(&format!(
                "- {}: {}\n  Parameters: {}\n  Policies: {}\n\n",
                tool.name(),
                tool.description(),
                tool.parameters_schema(),
                policy_str
            ));
        }

        descriptions
    }
}
