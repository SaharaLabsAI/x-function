use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::policy_registry::PolicyInfo;

/// Compliance attestation quote from hypervisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceQuote {
    /// Tool name this quote is for
    pub tool_name: String,
    /// Compliance check result (approved/rejected)
    pub compliant: bool,
    /// The raw TEE attestation quote bytes
    pub quote_bytes: Vec<u8>,
    /// Hash of compliance check data that was attested
    pub compliance_hash: [u8; 32],
    /// Timestamp of quote generation
    pub timestamp: std::time::SystemTime,
}

/// Plan created by the agent for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPlan {
    /// The agent's system prompt
    pub system_prompt: String,
    /// The user's query/input
    pub user_query: String,
    /// The agent's thought process (chain of thought)
    pub thought_process: Vec<ThoughtStep>,
    /// List of tools the agent intends to use
    pub intended_tool_calls: Vec<ToolCall>,
}

/// A step in the agent's reasoning process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtStep {
    /// Step number in the reasoning chain
    pub step: usize,
    /// The reasoning or analysis at this step
    pub content: String,
    /// Timestamp of this thought
    pub timestamp: std::time::SystemTime,
}

/// A tool call made by the agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique identifier for this tool call
    pub id: Uuid,
    /// Name of the tool being called
    pub tool_name: String,
    /// Arguments passed to the tool (JSON-serialized)
    pub arguments: String,
    /// Timestamp of the call
    pub timestamp: std::time::SystemTime,
    /// Compliance attestation quote from hypervisor (attached after compliance check)
    pub compliance_quote: Option<ComplianceQuote>,
}

/// Result from a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// ID of the corresponding tool call
    pub call_id: Uuid,
    /// Whether the tool execution was successful
    pub success: bool,
    /// The result data (JSON-serialized)
    pub result: String,
    /// Error message if execution failed
    pub error: Option<String>,
    /// Whether the compliance quote was verified by the tool
    pub quote_verified: bool,
}

/// A tool that can be used by the agent
pub trait Tool: Send + Sync {
    /// Name of the tool
    fn name(&self) -> &str;
    
    /// Description of what the tool does
    fn description(&self) -> &str;
    
    /// JSON schema for the tool's parameters
    fn parameters_schema(&self) -> serde_json::Value;
    
    /// Execute the tool with given arguments and compliance quote
    fn execute(&self, arguments: &str, compliance_quote: Option<&ComplianceQuote>) -> Result<String, String>;
    
    /// Get the policy IDs for this tool (many-to-many mapping)
    fn policy_ids(&self) -> Vec<String>;
    
    /// Get the policy information (ID and name) for this tool
    fn policy_info(&self) -> Vec<PolicyInfo>;
}

/// Complete execution trace of an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecution {
    /// Session ID for this execution
    pub session_id: Uuid,
    /// Plan created before execution
    pub plan: AgentPlan,
    /// Tool calls made during execution
    pub tool_calls: Vec<ToolCall>,
    /// Results from tool executions
    pub tool_results: Vec<ToolResult>,
    /// Final response from the agent
    pub final_response: String,
    /// Total execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Result of a compliance check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    /// Whether the agent's usage is compliant
    pub compliant: bool,
    /// Reason for the compliance decision
    pub reason: String,
    /// Hash of the policy used for checking
    pub policy_hash: String,
    /// Hash of the plan checked
    pub plan_hash: String,
}
