use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::SystemTime;
use tracing::{debug, info};
use uuid::Uuid;

use super::quote_utils::generate_compliance_quote;
use super::tools::ToolRegistry;
use super::types::{AgentPlan, AgentExecution, ThoughtStep, ToolCall, ToolResult};

/// Configuration for the crypto agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoAgentConfig {
    /// System prompt for the agent
    pub system_prompt: String,
    /// Maximum number of tool calls per query
    pub max_tool_calls: usize,
    /// Temperature for LLM
    pub temperature: f32,
    /// Maximum tokens for LLM response
    pub max_tokens: u32,
}

impl Default for CryptoAgentConfig {
    fn default() -> Self {
        Self {
            system_prompt: DEFAULT_SYSTEM_PROMPT.to_string(),
            temperature: 0.7,
            max_tokens: 2000,
            max_tool_calls: 10,
        }
    }
}

const DEFAULT_SYSTEM_PROMPT: &str = r#"You are a synthetic cryptocurrency research assistant. You can answer questions about cryptocurrencies and use various synthetic tools to gather information.

When answering questions:
1. Think step-by-step about what information you need
2. Use available synthetic tools to gather information
3. Provide clear, concise answers based on the tool results, even though the tool responses are synthetic
4. Always cite the data sources (tools) you used

After gathering information, provide a comprehensive answer to the user's question."#;

/// Crypto agent that answers crypto-related questions
pub struct CryptoAgent {
    config: CryptoAgentConfig,
    tool_registry: ToolRegistry,
}

impl CryptoAgent {
    /// Create a new crypto agent with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(CryptoAgentConfig::default())
    }

    /// Create a new crypto agent with custom configuration
    pub fn with_config(config: CryptoAgentConfig) -> Result<Self> {
        Ok(Self {
            config,
            tool_registry: ToolRegistry::new_crypto_tools()
                .map_err(|e| anyhow!("Failed to initialize tool registry: {}", e))?,
        })
    }

    /// Get the agent's system prompt (for compliance checking)
    pub fn system_prompt(&self) -> &str {
        &self.config.system_prompt
    }

    /// Process a user query and return plan with LLM-based planning
    /// This allows the hypervisor to perform compliance checks
    pub async fn plan_execution(
        &self,
        user_query: &str,
        openai_api_key: &str,
    ) -> Result<AgentPlan> {
        info!("Planning execution for query: {}", user_query);

        // Use LLM to plan tool usage
        let (thought_process, intended_tool_calls) = self
            .llm_based_planning(user_query, openai_api_key)
            .await?;

        Ok(AgentPlan {
            system_prompt: self.config.system_prompt.clone(),
            user_query: user_query.to_string(),
            thought_process,
            intended_tool_calls,
        })
    }

    /// Execute the agent with the given query
    /// Returns the complete execution trace
    /// This version performs per-tool compliance checking (deterministic only)
    pub async fn execute_with_compliance(
        &self,
        user_query: &str,
        session_id: Uuid,
        openai_api_key: &str,
        compliance_checker: &super::compliance::ComplianceChecker,
    ) -> Result<AgentExecution> {
        self.execute_with_compliance_internal(user_query, session_id, openai_api_key, compliance_checker, false)
            .await
    }

    /// Execute the agent with the given query
    /// Returns the complete execution trace
    /// This version performs per-tool compliance checking with LLM support
    pub async fn execute_with_llm_compliance(
        &self,
        user_query: &str,
        session_id: Uuid,
        openai_api_key: &str,
        compliance_checker: &super::compliance::ComplianceChecker,
    ) -> Result<AgentExecution> {
        self.execute_with_compliance_internal(user_query, session_id, openai_api_key, compliance_checker, true)
            .await
    }

    /// Internal execution method with optional LLM compliance
    async fn execute_with_compliance_internal(
        &self,
        user_query: &str,
        session_id: Uuid,
        openai_api_key: &str,
        compliance_checker: &super::compliance::ComplianceChecker,
        use_llm_compliance: bool,
    ) -> Result<AgentExecution> {
        let start_time = std::time::Instant::now();

        info!(
            session_id = %session_id, 
            use_llm_compliance = use_llm_compliance,
            "Starting agent execution with compliance"
        );

        // Phase 1: LLM-based planning
        let plan = self.plan_execution(user_query, openai_api_key).await?;

        // Phase 2: Per-tool compliance checking by hypervisor with attestation quote generation
        let mut approved_tool_calls = Vec::new();
        let mut rejected_tool_calls = Vec::new();
        let mut approved_policies = std::collections::HashMap::new(); // tool_name -> policy_texts

        for tool_call in &plan.intended_tool_calls {
            // Get the tool to find its policies
            if let Some(tool) = self.tool_registry.get_tool(&tool_call.tool_name) {
                let policy_ids = tool.policy_ids();
                
                // Check compliance for this specific tool call against all its policies
                let compliance_result = if use_llm_compliance {
                    compliance_checker.check_tool_compliance_async(
                        &tool_call.tool_name,
                        user_query,
                        &tool_call.arguments,
                        Some(openai_api_key),
                    )
                    .await
                } else {
                    compliance_checker.check_tool_compliance(
                        &tool_call.tool_name,
                        user_query,
                        &tool_call.arguments,
                    )
                };

                match compliance_result {
                    Ok(()) => {
                        debug!("Tool call '{}' approved by policies {:?}", tool_call.tool_name, policy_ids);
                        
                        // Generate TEE attestation quote for this compliance check
                        // The quote can include a nonce by the requested tools that guards against replay attacks (not implemented)
                        // It can be further signed by the requesting agent's key if needed (not implemented)
                        let compliance_quote = match generate_compliance_quote(
                            &tool_call.tool_name,
                            true, // approved
                            &policy_ids,
                            user_query,
                            &tool_call.arguments,
                        ) {
                            Ok(quote) => Some(quote),
                            Err(e) => {
                                info!(
                                    tool_name = %tool_call.tool_name,
                                    error = %e,
                                    "Failed to generate attestation quote, proceeding without quote"
                                );
                                None
                            }
                        };
                        
                        // Create tool call with attestation quote
                        let mut tool_call_with_quote = tool_call.clone();
                        tool_call_with_quote.compliance_quote = compliance_quote;
                        approved_tool_calls.push(tool_call_with_quote);
                        
                        // Collect policy texts for this approved tool
                        let mut policy_texts = Vec::new();
                        for policy_id in &policy_ids {
                            if let Some(policy) = compliance_checker.policies().iter().find(|p| &p.id == policy_id) {
                                policy_texts.push(format!("{} ({}): {}", policy.id, policy.name, policy.text));
                            }
                        }
                        approved_policies.insert(tool_call.tool_name.clone(), policy_texts);
                    }
                    Err(reason) => {
                        info!(
                            tool_name = %tool_call.tool_name,
                            tool_call_id = %tool_call.id,
                            policy_ids = ?policy_ids,
                            reason = %reason,
                            arguments = %tool_call.arguments,
                            llm_compliance = use_llm_compliance,
                            "Tool call rejected by compliance policy"
                        );
                        rejected_tool_calls.push((tool_call.clone(), reason));
                    }
                }
            } else {
                info!(
                    tool_name = %tool_call.tool_name,
                    tool_call_id = %tool_call.id,
                    arguments = %tool_call.arguments,
                    "Tool call rejected: tool not found in registry"
                );
                rejected_tool_calls.push((
                    tool_call.clone(),
                    format!("Tool '{}' not found", tool_call.tool_name),
                ));
            }
        }        // Log summary of compliance check results
        info!(
            session_id = %session_id,
            total_tools = plan.intended_tool_calls.len(),
            approved = approved_tool_calls.len(),
            rejected = rejected_tool_calls.len(),
            "Compliance check complete"
        );

        // Phase 3: Execute approved tool calls only
        let mut tool_results = Vec::new();
        for tool_call in &approved_tool_calls {
            debug!("Executing approved tool call: {}", tool_call.tool_name);
            let result = self.tool_registry.execute_tool_call(tool_call);
            tool_results.push(result);
        }

        // Add "rejected" results for rejected tools
        for (tool_call, reason) in &rejected_tool_calls {
            tool_results.push(ToolResult {
                call_id: tool_call.id,
                success: false,
                result: String::new(),
                error: Some(format!("Policy compliance failed: {}", reason)),
                quote_verified: false,
            });
        }

        // Phase 4: Generate final response with context of what was approved/rejected
        let final_response = self
            .generate_final_response_with_compliance(
                user_query,
                &plan,
                &tool_results,
                &rejected_tool_calls,
                &approved_policies,
                openai_api_key,
            )
            .await?;

        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        // Clone intended tool calls before moving plan
        let intended_tool_calls = plan.intended_tool_calls.clone();

        Ok(AgentExecution {
            session_id,
            plan,
            tool_calls: intended_tool_calls,
            tool_results,
            final_response,
            execution_time_ms,
        })
    }

    /// LLM-based planning: ask the LLM to plan which tools to use
    async fn llm_based_planning(
        &self,
        user_query: &str,
        openai_api_key: &str,
    ) -> Result<(Vec<ThoughtStep>, Vec<ToolCall>)> {
        // Build planning prompt with tool descriptions
        let tool_descriptions = self.tool_registry.generate_tool_descriptions();
        
        let planning_prompt = format!(
            r#"You are an in-house synthetic assistant planning how to answer a question about cryptocurrencies with synthetic tools.

{}

User question: {}

Please analyze this question and plan which synthetic tools you need to use. For each tool you want to use, provide:
1. Your reasoning for why you need this tool
2. The exact tool call in JSON format

Respond in this format:
THOUGHT: [your reasoning]
TOOL_CALL: {{"tool": "tool_name_1", "arguments": {{"param": "value_1"}}}}
THOUGHT: [your reasoning]
TOOL_CALL: {{"tool": "tool_name_2", "arguments": {{"param": "value_2"}}}}

If no tools are needed, output only a THOUGHT explaining your reasoning.
THOUGHT: [your reasoning]

You can specify multiple THOUGHT/TOOL_CALL pairs if you need multiple tools.
"#,
            tool_descriptions, user_query
        );

        // Call OpenAI for planning
        let system_prompt = "You are a planning assistant that helps determine which synthetic tools to use.";
        
        info!("[LLM_PLANNING_CALL] Starting OpenAI planning call");
        debug!("[LLM_PLANNING_CALL] System prompt: {}", system_prompt);
        debug!("[LLM_PLANNING_CALL] User prompt {}", planning_prompt);
        
        let client = reqwest::Client::new();
        let request_body = json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": planning_prompt
                }
            ],
            "temperature": 0.3,
            "max_tokens": 1000
        });

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to call OpenAI for planning")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            info!("[LLM_PLANNING_CALL] API error: {}", error_text);
            return Err(anyhow!("OpenAI planning API error: {}", error_text));
        }

        let openai_response: serde_json::Value =
            response.json().await.context("Failed to parse OpenAI planning response")?;

        let planning_text = openai_response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid OpenAI planning response format"))?;
        
        info!("[LLM_PLANNING_CALL] Response received ({} chars)", planning_text.len());
        debug!("[LLM_PLANNING_CALL] Response: {}", planning_text);

        // Parse the planning response
        self.parse_planning_response(planning_text, user_query)
    }

    /// Parse the LLM's planning response into thought steps and tool calls
    fn parse_planning_response(
        &self,
        planning_text: &str,
        _user_query: &str,
    ) -> Result<(Vec<ThoughtStep>, Vec<ToolCall>)> {
        let mut thought_process = Vec::new();
        let mut tool_calls = Vec::new();
        let mut current_step = 1;

        for line in planning_text.lines() {
            let line = line.trim();
            
            if line.starts_with("THOUGHT:") {
                let thought = line.strip_prefix("THOUGHT:").unwrap_or("").trim();
                if !thought.is_empty() {
                    thought_process.push(ThoughtStep {
                        step: current_step,
                        content: thought.to_string(),
                        timestamp: SystemTime::now(),
                    });
                    current_step += 1;
                }
            } else if line.starts_with("TOOL_CALL:") {
                let tool_json = line.strip_prefix("TOOL_CALL:").unwrap_or("").trim();
                if let Ok(tool_spec) = serde_json::from_str::<serde_json::Value>(tool_json) {
                    if let (Some(tool_name), Some(arguments)) = (
                        tool_spec["tool"].as_str(),
                        tool_spec.get("arguments"),
                    ) {
                        tool_calls.push(ToolCall {
                            id: Uuid::now_v7(),
                            tool_name: tool_name.to_string(),
                            arguments: arguments.to_string(),
                            timestamp: SystemTime::now(),
                            compliance_quote: None, // Quote will be added after compliance check
                        });
                    }
                }
            }
        }

        // If no tool calls were parsed, return empty planning
        if tool_calls.is_empty() {
            debug!("LLM planning produced no tool calls");
        }

        Ok((thought_process, tool_calls))
    }



    /// Generate final response with compliance awareness
    async fn generate_final_response_with_compliance(
        &self,
        user_query: &str,
        _plan: &AgentPlan,
        tool_results: &[ToolResult],
        _rejected_tools: &[(ToolCall, String)],
        approved_policies: &std::collections::HashMap<String, Vec<String>>,
        openai_api_key: &str,
    ) -> Result<String> {
        // Build policy context for approved tools
        let mut policy_context = String::from("\n\nAPPLICABLE POLICIES (You MUST follow these policies in your response):\n");
        let mut all_policy_texts = std::collections::HashSet::new();
        
        for (tool_name, policies) in approved_policies {
            if !policies.is_empty() {
                policy_context.push_str(&format!("\nFor tool '{}':\n", tool_name));
                for policy_text in policies {
                    policy_context.push_str(&format!("  - {}\n", policy_text));
                    all_policy_texts.insert(policy_text.clone());
                }
            }
        }
        
        if all_policy_texts.is_empty() {
            policy_context = String::from("\n\nNo specific policies apply to the approved tools.\n");
        }
        
        // Build context from tool results
        let mut tool_context = String::from("\n\nTool Results:\n");
        let mut had_rejections = false;

        for (i, result) in tool_results.iter().enumerate() {
            if result.success {
                tool_context.push_str(&format!("{}. SUCCESS: {}\n", i + 1, result.result));
            } else {
                had_rejections = true;
                let error_msg = result.error.as_deref().unwrap_or("Unknown error");
                if error_msg.contains("Policy compliance failed") {
                    tool_context.push_str(&format!(
                        "{}. REJECTED (Policy): Tool use was rejected by compliance policy.\n",
                        i + 1
                    ));
                } else {
                    tool_context.push_str(&format!("{}. ERROR: {}\n", i + 1, error_msg));
                }
            }
        }

        // Add instructions on how to handle rejections
        let rejection_guidance = if had_rejections {
            "\n\nIMPORTANT: Some tool calls were rejected by compliance policies. \
             If critical tools were rejected and you cannot answer the question without them, \
             you MUST respond with 'IMPOSSIBLE: [reason]' explaining why you cannot complete the request. \
             Otherwise, answer based on the available tool results."
        } else {
            ""
        };

        // Build the prompt for final response
        let prompt = format!(
            "{}\n\nUser Question: {}\n\n{}{}{}\n\n\
            Based on the available data, please provide a clear answer to the user's question. \
            CRITICAL: You MUST strictly follow all applicable policies listed above. \
            If you cannot answer due to policy restrictions, say so clearly.",
            self.config.system_prompt, user_query, policy_context, tool_context, rejection_guidance
        );

        info!("[LLM_RESPONSE_CALL] Starting OpenAI response generation call");
        debug!("[LLM_RESPONSE_CALL] System prompt: {}", self.config.system_prompt);
        debug!("[LLM_RESPONSE_CALL] User prompt: {}", prompt);
        debug!("[LLM_RESPONSE_CALL] Temperature: {}, Max tokens: {}", 
               self.config.temperature, self.config.max_tokens);

        // Call OpenAI API
        let client = reqwest::Client::new();
        let request_body = json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "system",
                    "content": self.config.system_prompt
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": self.config.temperature,
            "max_tokens": self.config.max_tokens
        });

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to call OpenAI API")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            info!("[LLM_RESPONSE_CALL] API error: {}", error_text);
            return Err(anyhow!("OpenAI API error: {}", error_text));
        }

        let openai_response: serde_json::Value =
            response.json().await.context("Failed to parse OpenAI response")?;

        let response_text = openai_response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid OpenAI response format"))?
            .to_string();
        
        info!("[LLM_RESPONSE_CALL] Response received ({} chars)", response_text.len());
        debug!("[LLM_RESPONSE_CALL] Response: {}", response_text);

        Ok(response_text)
    }
}

impl Default for CryptoAgent {
    fn default() -> Self {
        Self::new().expect("Failed to initialize CryptoAgent")
    }
}
