use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::agent::types::{AgentPlan, ComplianceResult, ToolCall};

/// Compliance checking method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceMethod {
    /// Deterministic & heuristic keyword matching
    Deterministic,
    /// LLM-based compliance check
    LLMBased,
}

/// LLM compliance check result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LLMComplianceResult {
    /// Whether the check passed
    pub compliant: bool,
    /// Explanation from the LLM
    pub explanation: String,
}

impl LLMComplianceResult {
    /// Check if the result is compliant
    pub fn is_compliant(&self) -> bool {
        self.compliant
    }
}

/// A policy that defines acceptable agent behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Unique identifier for the policy (e.g., "L1", "L2", etc.)
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Policy text/description
    pub text: String,
    /// Compliance checking methods for this policy
    pub methods: Vec<PolicyMethod>,
}

/// A compliance checking method for a policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyMethod {
    /// Method type
    pub method: ComplianceMethod,
    /// Method-specific rules
    pub rules: Vec<PolicyRule>,
}

/// A single policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule identifier
    pub id: String,
    /// Rule type
    pub rule_type: PolicyRuleType,
    /// Rule parameters
    pub parameters: serde_json::Value,
}

/// Types of policy rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PolicyRuleType {
    /// Restrict query content (keywords)
    ProhibitedKeywords { keywords: Vec<String> },
    /// Require certain patterns to be absent
    RequiredAbsentPatterns { patterns: Vec<String> },
    /// Limit output size or raw data dumps
    OutputRestriction {
        max_raw_items: Option<usize>,
        require_aggregation: bool,
    },
    /// Check for prohibited identity inference
    NoIdentityInference {
        prohibited_terms: Vec<String>,
    },
    /// Require source attribution
    RequireAttribution {
        require_source: bool,
        require_timestamp: bool,
    },
    /// Custom LLM-based compliance check
    LLMCompliance {
        check_prompt: String,
    },
}

/// Compliance checker for agent executions
pub struct ComplianceChecker {
    policies: Vec<Policy>,
    /// Many-to-many mapping: tool_name -> list of policy IDs
    tool_policy_map: std::collections::HashMap<String, Vec<String>>,
}

impl ComplianceChecker {
    /// Create a new compliance checker with given policies and tool-policy mapping
    pub fn new(
        policies: Vec<Policy>,
        tool_policy_map: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            policies,
            tool_policy_map,
        }
    }

    /// Create a default compliance checker with the new L1-L4 policies and T1-T4 tool mappings
    pub fn default_crypto_policy() -> Self {
        let registry = super::policy_registry::PolicyRegistry::default_crypto_policy();
        let (policies, tool_policy_map) = registry.clone_data();
        Self {
            policies,
            tool_policy_map,
        }
    }

    /// Get policy IDs for a given tool
    pub fn get_policy_ids_for_tool(&self, tool_name: &str) -> Vec<String> {
        self.tool_policy_map
            .get(tool_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if plan complies with all policies
    pub fn check_compliance(&self, plan: &AgentPlan) -> Result<ComplianceResult> {
        // Hash the plan
        let plan_hash = self.hash_plan(plan);

        // Hash the policies
        let policy_hash = self.hash_policies();

        // Check each policy
        for policy in &self.policies {
            for method in &policy.methods {
                // Only check deterministic methods in this function
                // LLM-based checks would be done separately
                if method.method == ComplianceMethod::Deterministic {
                    for rule in &method.rules {
                        if let Err(reason) = self.check_rule(rule, plan, None) {
                            return Ok(ComplianceResult {
                                compliant: false,
                                reason: format!("Policy '{}' ({}) rule '{}' violated: {}",
                                    policy.id, policy.name, rule.id, reason),
                                policy_hash: const_hex::encode(policy_hash),
                                plan_hash: const_hex::encode(plan_hash),
                            });
                        }
                    }
                }
            }
        }

        Ok(ComplianceResult {
            compliant: true,
            reason: "All policy checks passed".to_string(),
            policy_hash: const_hex::encode(policy_hash),
            plan_hash: const_hex::encode(plan_hash),
        })
    }

    /// Check compliance for a specific tool call against its policies
    /// Returns Ok(()) if compliant, Err(reason) if not
    pub fn check_tool_compliance(
        &self,
        tool_name: &str,
        user_query: &str,
        tool_arguments: &str,
    ) -> Result<(), String> {
        // Get policy IDs for this tool
        let policy_ids = self.get_policy_ids_for_tool(tool_name);

        if policy_ids.is_empty() {
            // No policies for this tool, allow it
            return Ok(());
        }

        // Check each policy
        for policy_id in &policy_ids {
            let policy = self
                .policies
                .iter()
                .find(|p| p.id == *policy_id)
                .ok_or_else(|| format!("Policy '{}' not found for tool '{}'", policy_id, tool_name))?;

            // Check each method (only deterministic for now)
            for method in &policy.methods {
                if method.method == ComplianceMethod::Deterministic {
                    for rule in &method.rules {
                        // Create temporary plan with just this tool call
                        let temp_plan = AgentPlan {
                            system_prompt: String::new(),
                            user_query: user_query.to_string(),
                            thought_process: vec![],
                            intended_tool_calls: vec![ToolCall {
                                id: uuid::Uuid::now_v7(),
                                tool_name: tool_name.to_string(),
                                arguments: tool_arguments.to_string(),
                                timestamp: std::time::SystemTime::now(),
                                compliance_quote: None,
                            }],
                        };

                        if let Err(reason) = self.check_rule(rule, &temp_plan, None) {
                            return Err(format!(
                                "Tool '{}' policy '{}' ({}) rule '{}' violated: {}",
                                tool_name, policy.id, policy.name, rule.id, reason
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check compliance for a specific tool call against its policies (with LLM support)
    /// Returns Ok(()) if compliant, Err(reason) if not
    pub async fn check_tool_compliance_async(
        &self,
        tool_name: &str,
        user_query: &str,
        tool_arguments: &str,
        openai_api_key: Option<&str>,
    ) -> Result<(), String> {
        // Get policy IDs for this tool
        let policy_ids = self.get_policy_ids_for_tool(tool_name);

        if policy_ids.is_empty() {
            // No policies for this tool, allow it
            return Ok(());
        }

        // Check each policy
        for policy_id in &policy_ids {
            let policy = self
                .policies
                .iter()
                .find(|p| p.id == *policy_id)
                .ok_or_else(|| format!("Policy '{}' not found for tool '{}'", policy_id, tool_name))?;

            // Check each method
            for method in &policy.methods {
                match method.method {
                    ComplianceMethod::Deterministic => {
                        for rule in &method.rules {
                            // Create temporary plan with just this tool call
                            let temp_plan = AgentPlan {
                                system_prompt: String::new(),
                                user_query: user_query.to_string(),
                                thought_process: vec![],
                                intended_tool_calls: vec![ToolCall {
                                    id: uuid::Uuid::now_v7(),
                                    tool_name: tool_name.to_string(),
                                    arguments: tool_arguments.to_string(),
                                    timestamp: std::time::SystemTime::now(),
                                    compliance_quote: None,
                                }],
                            };

                            if let Err(reason) = self.check_rule(rule, &temp_plan, None) {
                                return Err(format!(
                                    "Tool '{}' policy '{}' ({}) rule '{}' violated: {}",
                                    tool_name, policy.id, policy.name, rule.id, reason
                                ));
                            }
                        }
                    }
                    ComplianceMethod::LLMBased => {
                        if let Some(api_key) = openai_api_key {
                            for rule in &method.rules {
                                if let Err(reason) = self
                                    .check_llm_rule(rule, &policy.text, tool_name, user_query, tool_arguments, api_key)
                                    .await
                                {
                                    return Err(format!(
                                        "Tool '{}' policy '{}' ({}) LLM rule '{}' violated: {}",
                                        tool_name, policy.id, policy.name, rule.id, reason
                                    ));
                                }
                            }
                        }
                        // If no API key provided, skip LLM checks
                    }
                }
            }
        }

        Ok(())
    }

    /// Check an LLM-based rule
    async fn check_llm_rule(
        &self,
        rule: &PolicyRule,
        policy_text: &str,
        tool_name: &str,
        user_query: &str,
        tool_arguments: &str,
        openai_api_key: &str,
    ) -> Result<(), String> {
        use tracing::{info, debug};
        
        if let PolicyRuleType::LLMCompliance { check_prompt } = &rule.rule_type {
            let context = format!(
                "Tool: {}\nUser Query: {}\nTool Arguments: {}",
                tool_name, user_query, tool_arguments
            );

            let full_prompt = format!(
                "Policy: {}\n\n{}\n\nContext:\n{}\n\nPlease respond in JSON format with two fields:\n1. \"compliant\": true or false\n2. \"explanation\": a brief explanation of your decision\n\nExample: {{\"compliant\": true, \"explanation\": \"The query does not violate the policy because...\"}}",
                policy_text, check_prompt, context
            );

            info!("[LLM_COMPLIANCE_CHECK] Starting OpenAI compliance check call");
            debug!("[LLM_COMPLIANCE_CHECK] Rule ID: {}", rule.id);
            debug!("[LLM_COMPLIANCE_CHECK] Tool: {}", tool_name);
            debug!("[LLM_COMPLIANCE_CHECK] Full prompt: {}", full_prompt);

            // Call OpenAI API
            let client = reqwest::Client::new();
            let request_body = serde_json::json!({
                "model": "gpt-4o",
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a compliance checker that examines whether the tool use of the LLM agent complies with the policy set by the tool owner. Respond with a JSON object containing 'compliant' (boolean) and 'explanation' (string)."
                    },
                    {
                        "role": "user",
                        "content": full_prompt
                    }
                ],
                "temperature": 0.0,
                "max_tokens": 150,
                "response_format": { "type": "json_object" }
            });

            let response = client
                .post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", openai_api_key))
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
                .await
                .map_err(|e| format!("Failed to call OpenAI API: {}", e))?;

            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                info!("[LLM_COMPLIANCE_CHECK] API error: {}", error_text);
                return Err(format!("OpenAI API error: {}", error_text));
            }

            let openai_response: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

            let llm_result = openai_response["choices"][0]["message"]["content"]
                .as_str()
                .ok_or_else(|| "Invalid OpenAI response format".to_string())?
                .trim();

            info!("[LLM_COMPLIANCE_CHECK] Response received ({} chars)", llm_result.len());
            debug!("[LLM_COMPLIANCE_CHECK] Response: {}", llm_result);

            // Parse the JSON result
            let compliance_result: LLMComplianceResult = serde_json::from_str(llm_result)
                .map_err(|e| format!("Failed to parse LLM compliance result: {}. Response: {}", e, llm_result))?;

            info!("[LLM_COMPLIANCE_CHECK] Compliance result: compliant={}, explanation='{}'", 
                  compliance_result.compliant, compliance_result.explanation);

            if !compliance_result.is_compliant() {
                return Err(format!("LLM compliance check failed: {}", compliance_result.explanation));
            }

            Ok(())
        } else {
            Ok(())
        }
    }

    /// Check a single rule against plan
    /// Optional response parameter for checking output-related rules
    fn check_rule(&self, rule: &PolicyRule, plan: &AgentPlan, response: Option<&str>) -> Result<(), String> {
        match &rule.rule_type {
            PolicyRuleType::ProhibitedKeywords { keywords } => {
                let query_lower = plan.user_query.to_lowercase();
                let system_prompt_lower = plan.system_prompt.to_lowercase();

                for keyword in keywords {
                    let keyword_lower = keyword.to_lowercase();
                    if query_lower.contains(&keyword_lower) {
                        return Err(format!("Prohibited keyword '{}' found in user query", keyword));
                    }
                    if system_prompt_lower.contains(&keyword_lower) {
                        return Err(format!(
                            "Prohibited keyword '{}' found in system prompt",
                            keyword
                        ));
                    }

                    // Also check tool arguments
                    for tool_call in &plan.intended_tool_calls {
                        let args_lower = tool_call.arguments.to_lowercase();
                        if args_lower.contains(&keyword_lower) {
                            return Err(format!(
                                "Prohibited keyword '{}' found in tool arguments",
                                keyword
                            ));
                        }
                    }
                }
                Ok(())
            }
            PolicyRuleType::RequiredAbsentPatterns { patterns } => {
                let query_lower = plan.user_query.to_lowercase();

                for pattern in patterns {
                    let pattern_lower = pattern.to_lowercase();
                    if query_lower.contains(&pattern_lower) {
                        return Err(format!("Prohibited pattern '{}' found", pattern));
                    }
                }
                Ok(())
            }
            PolicyRuleType::OutputRestriction { max_raw_items: _, require_aggregation } => {
                // This check would typically be done on the response
                // For now, we'll just validate the rule exists
                if let Some(resp) = response {
                    if *require_aggregation {
                        // Simple heuristic: check if response contains aggregation keywords
                        let resp_lower = resp.to_lowercase();
                        let has_aggregation = resp_lower.contains("total")
                            || resp_lower.contains("average")
                            || resp_lower.contains("summary")
                            || resp_lower.contains("aggregated");

                        if !has_aggregation {
                            return Err("Response should contain aggregated data".to_string());
                        }
                    }
                }
                Ok(())
            }
            PolicyRuleType::NoIdentityInference { prohibited_terms } => {
                let query_lower = plan.user_query.to_lowercase();

                for term in prohibited_terms {
                    let term_lower = term.to_lowercase();
                    if query_lower.contains(&term_lower) {
                        return Err(format!("Identity inference term '{}' found", term));
                    }
                }

                // Also check in response if provided
                if let Some(resp) = response {
                    let resp_lower = resp.to_lowercase();
                    for term in prohibited_terms {
                        let term_lower = term.to_lowercase();
                        if resp_lower.contains(&term_lower) {
                            return Err(format!("Identity inference term '{}' found in response", term));
                        }
                    }
                }
                Ok(())
            }
            PolicyRuleType::RequireAttribution { require_source, require_timestamp } => {
                // This check is typically done on the response
                if let Some(resp) = response {
                    let resp_lower = resp.to_lowercase();

                    if *require_source {
                        let has_source = resp_lower.contains("according to")
                            || resp_lower.contains("source:")
                            || resp_lower.contains("from");

                        if !has_source {
                            return Err("Response must include source attribution".to_string());
                        }
                    }

                    if *require_timestamp {
                        let has_timestamp = resp_lower.contains("as of")
                            || resp_lower.contains("timestamp")
                            || resp_lower.contains("utc")
                            || resp_lower.contains("time:");

                        if !has_timestamp {
                            return Err("Response must include timestamp".to_string());
                        }
                    }
                }
                Ok(())
            }
            PolicyRuleType::LLMCompliance { .. } => {
                // LLM-based compliance would require calling an LLM
                // For now, we'll just pass this check
                // In a real implementation, this would call an LLM and check the result
                Ok(())
            }
        }
    }

    /// Hash plan for attestation
    fn hash_plan(&self, plan: &AgentPlan) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(plan.system_prompt.as_bytes());
        hasher.update(plan.user_query.as_bytes());

        // Include thought process
        for step in &plan.thought_process {
            hasher.update(step.content.as_bytes());
        }

        // Include tool calls
        for call in &plan.intended_tool_calls {
            hasher.update(call.tool_name.as_bytes());
            hasher.update(call.arguments.as_bytes());
        }

        hasher.finalize().into()
    }

    /// Hash policies for attestation
    fn hash_policies(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();

        for policy in &self.policies {
            hasher.update(policy.id.as_bytes());
            hasher.update(policy.name.as_bytes());
            hasher.update(policy.text.as_bytes());

            for method in &policy.methods {
                let method_json = serde_json::to_string(&method.method).unwrap_or_default();
                hasher.update(method_json.as_bytes());

                for rule in &method.rules {
                    hasher.update(rule.id.as_bytes());
                    let rule_json = serde_json::to_string(&rule.rule_type).unwrap_or_default();
                    hasher.update(rule_json.as_bytes());
                }
            }
        }

        hasher.finalize().into()
    }

    /// Get the policies
    pub fn policies(&self) -> &[Policy] {
        &self.policies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy_structure() {
        let checker = ComplianceChecker::default_crypto_policy();

        // Check we have 4 policies (L1-L4)
        assert_eq!(checker.policies().len(), 4);

        // Check policy IDs
        let policy_ids: Vec<String> = checker.policies().iter().map(|p| p.id.clone()).collect();
        assert!(policy_ids.contains(&"L1".to_string()));
        assert!(policy_ids.contains(&"L2".to_string()));
        assert!(policy_ids.contains(&"L3".to_string()));
        assert!(policy_ids.contains(&"L4".to_string()));
    }

    #[test]
    fn test_tool_policy_mapping() {
        let checker = ComplianceChecker::default_crypto_policy();

        // T1: PriceFeedTool -> L1
        let t1_policies = checker.get_policy_ids_for_tool("PriceFeedTool");
        assert_eq!(t1_policies, vec!["L1"]);

        // T2: OnChainHistoryTool -> L1, L2, L3
        let t2_policies = checker.get_policy_ids_for_tool("OnChainHistoryTool");
        assert_eq!(t2_policies, vec!["L1", "L2", "L3"]);

        // T3: SentimentTool -> L1, L4
        let t3_policies = checker.get_policy_ids_for_tool("SentimentTool");
        assert_eq!(t3_policies, vec!["L1", "L4"]);

        // T4: PortfolioTool -> L1, L2, L3, L4
        let t4_policies = checker.get_policy_ids_for_tool("PortfolioTool");
        assert_eq!(t4_policies, vec!["L1", "L2", "L3", "L4"]);
    }

    #[test]
    fn test_compliance_l1_prohibited_keywords() {
        let checker = ComplianceChecker::default_crypto_policy();

        let plan = AgentPlan {
            system_prompt: "Test system prompt".to_string(),
            user_query: "You should buy Bitcoin now".to_string(),
            thought_process: vec![],
            intended_tool_calls: vec![],
        };

        let result = checker.check_compliance(&plan).unwrap();
        assert!(!result.compliant);
        assert!(result.reason.contains("should buy"));
    }

    #[test]
    fn test_compliance_l3_identity_inference() {
        let checker = ComplianceChecker::default_crypto_policy();

        let plan = AgentPlan {
            system_prompt: "Test system prompt".to_string(),
            user_query: "This wallet belongs to Satoshi".to_string(),
            thought_process: vec![],
            intended_tool_calls: vec![],
        };

        let result = checker.check_compliance(&plan).unwrap();
        assert!(!result.compliant);
        assert!(result.reason.contains("belongs to"));
    }

    #[test]
    fn test_tool_compliance_check() {
        let checker = ComplianceChecker::default_crypto_policy();

        // Test PriceFeedTool with L1 violation
        let result = checker.check_tool_compliance(
            "PriceFeedTool",
            "You should buy this coin",
            r#"{"symbol": "BTC"}"#,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("should buy"));
    }

    #[test]
    fn test_llm_compliance_result() {
        let compliant = LLMComplianceResult {
            compliant: true,
            explanation: "Passes all checks".to_string(),
        };
        assert!(compliant.is_compliant());

        let non_compliant = LLMComplianceResult {
            compliant: false,
            explanation: "Violates policy".to_string(),
        };
        assert!(!non_compliant.is_compliant());
    }

    #[tokio::test]
    async fn test_llm_compliance_check_mock() {
        // This test verifies the LLM compliance check structure
        // In a real scenario, you would mock the OpenAI API or use a test API key
        let checker = ComplianceChecker::default_crypto_policy();

        // Test without API key (should skip LLM checks and pass)
        let result = checker
            .check_tool_compliance_async(
                "PriceFeedTool",
                "What is the price of Bitcoin?",
                r#"{"symbol": "BTC"}"#,
                None, // No API key
            )
            .await;

        assert!(result.is_ok());
    }
}
