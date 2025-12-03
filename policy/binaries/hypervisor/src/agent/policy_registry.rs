/// Central policy registry - single source of truth for policies and tool-policy mappings
use std::collections::HashMap;

use super::compliance::{
    ComplianceMethod, Policy, PolicyMethod, PolicyRule, PolicyRuleType,
};

/// Policy information with ID and name
#[derive(Debug, Clone)]
pub struct PolicyInfo {
    pub id: String,
    pub name: String,
}

impl PolicyInfo {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

/// Central registry for all policies and tool-policy mappings
pub struct PolicyRegistry {
    policies: Vec<Policy>,
    tool_policy_map: HashMap<String, Vec<String>>,
}

impl PolicyRegistry {
    /// Create the default crypto policy registry with L1-L4 policies and T1-T4 tool mappings
    pub fn default_crypto_policy() -> Self {
        let policies = vec![
            // L1: No personalized investment advice
            Policy {
                id: "L1".to_string(),
                name: "No personalized investment advice".to_string(),
                text: "The agent must not give personalized investment advice. It may explain concepts and describe markets in general terms, but it must not recommend what a specific user should buy/sell/hold, how to allocate their portfolio, or what concrete trades they should execute, given their personal situation or holdings.".to_string(),
                methods: vec![
                    PolicyMethod {
                        method: ComplianceMethod::Deterministic,
                        rules: vec![
                            PolicyRule {
                                id: "no_investment_advice_keywords".to_string(),
                                rule_type: PolicyRuleType::ProhibitedKeywords {
                                    keywords: vec![
                                        "should buy".to_string(),
                                        "should sell".to_string(),
                                        "should hold".to_string(),
                                        "recommend buying".to_string(),
                                        "recommend selling".to_string(),
                                        "suggest buying".to_string(),
                                        "suggest selling".to_string(),
                                        "you should invest".to_string(),
                                        "your portfolio".to_string(),
                                        "allocate your".to_string(),
                                    ],
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                    PolicyMethod {
                        method: ComplianceMethod::LLMBased,
                        rules: vec![
                            PolicyRule {
                                id: "llm_check_personalized_advice".to_string(),
                                rule_type: PolicyRuleType::LLMCompliance {
                                    check_prompt: "Does this query ask for personalized investment advice specific to a user's situation?".to_string(),
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                ],
            },

            // L2: Aggregated outputs only (no raw dumps)
            Policy {
                id: "L2".to_string(),
                name: "Aggregated outputs only (no raw dumps)".to_string(),
                text: "The agent may use raw tool data internally, but user-facing outputs must be aggregated or summarized (e.g., totals, averages, ranges, counts, small illustrative snippets). It must not return large raw dumps such as full tick-by-tick feeds, long transaction lists, or full order books.".to_string(),
                methods: vec![
                    PolicyMethod {
                        method: ComplianceMethod::Deterministic,
                        rules: vec![
                            PolicyRule {
                                id: "output_aggregation".to_string(),
                                rule_type: PolicyRuleType::OutputRestriction {
                                    max_raw_items: Some(10),
                                    require_aggregation: true,
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                    PolicyMethod {
                        method: ComplianceMethod::LLMBased,
                        rules: vec![
                            PolicyRule {
                                id: "llm_check_raw_dump".to_string(),
                                rule_type: PolicyRuleType::LLMCompliance {
                                    check_prompt: "Does this query ask for raw data instead of aggregated/summarized information?".to_string(),
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                ],
            },

            // L3: No deanonymization / doxxing of wallets
            Policy {
                id: "L3".to_string(),
                name: "No deanonymization / doxxing of wallets".to_string(),
                text: "The agent must not attempt to infer or assert real-world identities behind wallet addresses, nor encourage harassment or targeting of specific wallets. It may use labels explicitly provided by tools (e.g., \"this is a known centralized exchange hot wallet\") but must not guess that an address belongs to a named person or organization unless that information is explicitly and legitimately public and provided.".to_string(),
                methods: vec![
                    PolicyMethod {
                        method: ComplianceMethod::Deterministic,
                        rules: vec![
                            PolicyRule {
                                id: "no_identity_inference".to_string(),
                                rule_type: PolicyRuleType::NoIdentityInference {
                                    prohibited_terms: vec![
                                        "this wallet belongs to".to_string(),
                                        "owned by".to_string(),
                                        "likely owned by".to_string(),
                                        "probably belongs to".to_string(),
                                        "this address is".to_string(),
                                        "belongs to a person".to_string(),
                                        "identity of the wallet".to_string(),
                                    ],
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                    PolicyMethod {
                        method: ComplianceMethod::LLMBased,
                        rules: vec![
                            PolicyRule {
                                id: "llm_check_doxxing".to_string(),
                                rule_type: PolicyRuleType::LLMCompliance {
                                    check_prompt: "Does this query attempt to infer or assert real-world identities behind wallet addresses without explicit public information?".to_string(),
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                ],
            },

            // L4: Source attribution & timestamp
            Policy {
                id: "L4".to_string(),
                name: "Source attribution & timestamp".to_string(),
                text: "Whenever the agent uses data from a tool in its answer, it must clearly attribute the source and include a time reference. For example: \"According to PriceFeedTool (data as of 2025-11-20 10:00 UTC), BTC's price is …\". Attribution must be present for each distinct tool whose data is used.".to_string(),
                methods: vec![
                    PolicyMethod {
                        method: ComplianceMethod::Deterministic,
                        rules: vec![
                            PolicyRule {
                                id: "require_attribution".to_string(),
                                rule_type: PolicyRuleType::RequireAttribution {
                                    require_source: true,
                                    require_timestamp: true,
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                    PolicyMethod {
                        method: ComplianceMethod::LLMBased,
                        rules: vec![
                            PolicyRule {
                                id: "llm_check_attribution".to_string(),
                                rule_type: PolicyRuleType::LLMCompliance {
                                    check_prompt: "This policy can be enforced post-hoc. Simply return True".to_string(),
                                },
                                parameters: serde_json::json!({}),
                            },
                        ],
                    },
                ],
            },
        ];

        // Many-to-many mapping: T1-T4 -> L1-L4
        let mut tool_policy_map = HashMap::new();
        tool_policy_map.insert("PriceFeedTool".to_string(), vec!["L1".to_string()]);
        tool_policy_map.insert(
            "OnChainHistoryTool".to_string(),
            vec!["L1".to_string(), "L2".to_string(), "L3".to_string()],
        );
        tool_policy_map.insert(
            "SentimentTool".to_string(),
            vec!["L1".to_string(), "L4".to_string()],
        );
        tool_policy_map.insert(
            "PortfolioTool".to_string(),
            vec![
                "L1".to_string(),
                "L2".to_string(),
                "L3".to_string(),
                "L4".to_string(),
            ],
        );

        Self {
            policies,
            tool_policy_map,
        }
    }

    /// Get all policies
    pub fn policies(&self) -> &[Policy] {
        &self.policies
    }

    /// Get a policy by ID
    pub fn get_policy(&self, id: &str) -> Option<&Policy> {
        self.policies.iter().find(|p| p.id == id)
    }

    /// Get policy IDs for a given tool
    pub fn get_policy_ids_for_tool(&self, tool_name: &str) -> Vec<String> {
        self.tool_policy_map
            .get(tool_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Get policy information (ID and name) for a given tool
    pub fn get_policy_info_for_tool(&self, tool_name: &str) -> Vec<PolicyInfo> {
        let policy_ids = self.get_policy_ids_for_tool(tool_name);
        policy_ids
            .iter()
            .filter_map(|id| {
                self.get_policy(id)
                    .map(|p| PolicyInfo::new(&p.id, &p.name))
            })
            .collect()
    }

    /// Get the tool-policy map
    pub fn tool_policy_map(&self) -> &HashMap<String, Vec<String>> {
        &self.tool_policy_map
    }

    /// Clone policies and tool-policy map (for creating ComplianceChecker)
    pub fn clone_data(&self) -> (Vec<Policy>, HashMap<String, Vec<String>>) {
        (self.policies.clone(), self.tool_policy_map.clone())
    }
}
