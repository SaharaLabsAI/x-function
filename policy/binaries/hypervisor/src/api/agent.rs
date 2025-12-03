use aes_gcm_siv::aead::Aead;
use anyhow::{anyhow, Context};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};
use uuid::Uuid;

use crate::{
    agent::{AgentExecution, ComplianceChecker, ComplianceResult, CryptoAgent},
    error::HypervisorError,
    types::HypervisorState,
    utils::crypto,
};

pub(crate) fn api_register(router: Router<HypervisorState>) -> Router<HypervisorState> {
    router
        .route("/agent/query", post(query_agent))
        .route("/verifiable/agent/query", post(verifiable_query_agent))
}

/// Request to query the crypto agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentQueryRequest {
    /// Encrypted user query (hex-encoded)
    pub encrypted_query: String,
    /// User's public key (hex-encoded compressed SECP256K1 public key)
    pub public_key: String,
    /// Whether to use LLM-based compliance checking (default: false)
    #[serde(default)]
    pub use_llm_compliance: bool,
}

/// Response from agent query
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentQueryResponse {
    /// Session ID
    pub session_id: Uuid,
    /// Encrypted response (hex-encoded)
    pub encrypted_response: String,
    /// Nonce used for response encryption (hex-encoded)
    pub response_nonce: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Hash of the execution trace
    pub execution_hash: String,
    /// Full execution details (for hash verification)
    pub execution: AgentExecution,
}

/// Response from verifiable agent query
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiableAgentQueryResponse {
    /// Session ID
    pub session_id: Uuid,
    /// Encrypted response (hex-encoded)
    pub encrypted_response: String,
    /// Nonce used for response encryption (hex-encoded)
    pub response_nonce: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Hash of the execution trace
    pub execution_hash: String,
    /// TEE attestation quote (hex-encoded)
    pub quote: String,
    /// Compliance check result
    pub compliance: ComplianceResult,
    /// Full execution details (for hash verification)
    pub execution: AgentExecution,
}

/// Query the crypto agent (without verification)
#[tracing::instrument(skip(state, req), err)]
async fn query_agent(
    State(state): State<HypervisorState>,
    Json(req): Json<AgentQueryRequest>,
) -> Result<Json<AgentQueryResponse>, HypervisorError> {
    // Validate request
    validate_agent_request(&req)?;

    // Decode user's public key
    let user_pk = crypto::pk_from_hex(&req.public_key)
        .context(StatusCode::BAD_REQUEST)
        .context("decode request pubkey")?;

    // Get session keypair
    let (session_sk, session_id) = state
        .get_session_keypair(&user_pk)
        .ok_or(anyhow!("session not found"))
        .context(StatusCode::UNAUTHORIZED)?;

    // Create cipher for this session
    let cipher = crypto::create_encrypt_key(&session_sk, &user_pk, session_id)
        .context(StatusCode::INTERNAL_SERVER_ERROR)
        .context("create encrypt key")?;

    let msg_nonce = crypto::derive_msg_nonce(session_id);

    // Decrypt the query
    let decrypted_query = {
        let encrypted_bytes = const_hex::decode(&req.encrypted_query)
            .context(StatusCode::BAD_REQUEST)
            .context("invalid query hex")?;

        debug!(
            session_id = %session_id,
            public_key = %req.public_key,
            encrypted_len = encrypted_bytes.len(),
            "attempting to decrypt query"
        );

        let decrypted = cipher
            .decrypt(&msg_nonce, encrypted_bytes.as_slice())
            .map_err(|e| anyhow!(e.to_string()))
            .context(StatusCode::BAD_REQUEST)
            .context("decrypt query")?;

        String::from_utf8(decrypted)
            .context(StatusCode::BAD_REQUEST)
            .context("query isn't valid UTF-8")?
    };

    info!(
        session_id = %session_id,
        public_key = req.public_key,
        query_length = decrypted_query.len(),
        use_llm_compliance = req.use_llm_compliance,
        "processing crypto agent query"
    );

    // Get OpenAI API key
    let api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY not set")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Execute agent with per-tool compliance checking
    let agent = CryptoAgent::new()
        .context("Failed to initialize agent")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;
    let checker = ComplianceChecker::default_crypto_policy();
    
    let execution = if req.use_llm_compliance {
        agent
            .execute_with_llm_compliance(&decrypted_query, session_id, &api_key, &checker)
            .await
            .context("agent execution failed")
            .context(StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        agent
            .execute_with_compliance(&decrypted_query, session_id, &api_key, &checker)
            .await
            .context("agent execution failed")
            .context(StatusCode::INTERNAL_SERVER_ERROR)?
    };

    // Hash the execution
    let execution_hash = hash_execution(&execution);

    // Encrypt the response
    let response_nonce = crypto::derive_msg_nonce(execution.final_response.as_bytes());
    let encrypted_response = {
        let encrypted = cipher
            .encrypt(&response_nonce, execution.final_response.as_bytes())
            .map_err(|e| anyhow!(e.to_string()))
            .context("encrypt response")
            .context(StatusCode::INTERNAL_SERVER_ERROR)?;

        const_hex::encode(encrypted)
    };

    info!(
        session_id = %session_id,
        execution_time_ms = execution.execution_time_ms,
        status = "success",
        msg = "Agent query completed successfully"
    );

    Ok(Json(AgentQueryResponse {
        session_id,
        encrypted_response,
        response_nonce: const_hex::encode(response_nonce),
        execution_time_ms: execution.execution_time_ms,
        execution_hash: const_hex::encode(execution_hash),
        execution,
    }))
}

/// Query the crypto agent with verification and compliance check
#[tracing::instrument(skip(state, req), err)]
async fn verifiable_query_agent(
    State(state): State<HypervisorState>,
    Json(req): Json<AgentQueryRequest>,
) -> Result<Json<VerifiableAgentQueryResponse>, HypervisorError> {
    // Decode user's public key
    let user_pk = crypto::pk_from_hex(&req.public_key)
        .context(StatusCode::BAD_REQUEST)
        .context("decode request pubkey")?;

    // Get session keypair
    let (session_sk, session_id) = state
        .get_session_keypair(&user_pk)
        .ok_or(anyhow!("session not found"))
        .context(StatusCode::UNAUTHORIZED)?;

    // Create cipher for this session
    let cipher = crypto::create_encrypt_key(&session_sk, &user_pk, session_id)
        .context(StatusCode::INTERNAL_SERVER_ERROR)
        .context("create encrypt key")?;

    let msg_nonce = crypto::derive_msg_nonce(session_id);

    // Decrypt the query
    let decrypted_query = {
        let encrypted_bytes = const_hex::decode(&req.encrypted_query)
            .context(StatusCode::BAD_REQUEST)
            .context("invalid query hex")?;

        let decrypted = cipher
            .decrypt(&msg_nonce, encrypted_bytes.as_slice())
            .map_err(|e| anyhow!(e.to_string()))
            .context(StatusCode::BAD_REQUEST)
            .context("decrypt query")?;

        String::from_utf8(decrypted)
            .context(StatusCode::BAD_REQUEST)
            .context("query isn't valid UTF-8")?
    };

    info!(
        session_id = %session_id,
        public_key = req.public_key,
        query_length = decrypted_query.len(),
        use_llm_compliance = req.use_llm_compliance,
        "processing verifiable crypto agent query"
    );

    // Get OpenAI API key
    let api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY not set")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Execute agent with per-tool compliance checking
    let agent = CryptoAgent::new()
        .context("Failed to initialize agent")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;
    let checker = ComplianceChecker::default_crypto_policy();
    
    let execution = if req.use_llm_compliance {
        agent
            .execute_with_llm_compliance(&decrypted_query, session_id, &api_key, &checker)
            .await
            .context("agent execution failed")
            .context(StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        agent
            .execute_with_compliance(&decrypted_query, session_id, &api_key, &checker)
            .await
            .context("agent execution failed")
            .context(StatusCode::INTERNAL_SERVER_ERROR)?
    };

    // Generate compliance summary for attestation
    // (compliance already checked during execute_with_compliance)
    let compliance = generate_compliance_summary(&execution);

    // Hash the execution
    let execution_hash = hash_execution(&execution);

    // Generate attestation quote
    let quote = attest::get_quote(crate::utils::attest::generate_raw_report_from_hash(
        execution_hash,
    ))
    .context("get agent query quote")
    .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Encrypt the response
    let response_nonce = crypto::derive_msg_nonce(execution.final_response.as_bytes());
    let encrypted_response = {
        let encrypted = cipher
            .encrypt(&response_nonce, execution.final_response.as_bytes())
            .map_err(|e| anyhow!(e.to_string()))
            .context("encrypt response")
            .context(StatusCode::INTERNAL_SERVER_ERROR)?;

        const_hex::encode(encrypted)
    };

    info!(
        session_id = %session_id,
        execution_time_ms = execution.execution_time_ms,
        compliance = compliance.compliant,
        status = "success",
        msg = "Verifiable agent query completed successfully"
    );

    Ok(Json(VerifiableAgentQueryResponse {
        session_id,
        encrypted_response,
        response_nonce: const_hex::encode(response_nonce),
        execution_time_ms: execution.execution_time_ms,
        execution_hash: const_hex::encode(execution_hash),
        quote: const_hex::encode(quote.to_bytes()),
        compliance,
        execution,
    }))
}

/// Validate agent request
fn validate_agent_request(request: &AgentQueryRequest) -> Result<(), HypervisorError> {
    let validate = || -> anyhow::Result<()> {
        anyhow::ensure!(
            !request.encrypted_query.trim().is_empty(),
            "encrypted_query cannot be empty"
        );

        anyhow::ensure!(
            !request.public_key.trim().is_empty(),
            "public_key cannot be empty"
        );

        Ok(())
    };

    validate().context(StatusCode::BAD_REQUEST)?;

    Ok(())
}

/// Generate a compliance summary for a completed execution
/// Checks if any tool calls failed during execution
fn generate_compliance_summary(execution: &AgentExecution) -> ComplianceResult {
    let plan_hash = {
        let mut hasher = blake3::Hasher::new();
        hasher.update(execution.plan.system_prompt.as_bytes());
        hasher.update(execution.plan.user_query.as_bytes());
        for step in &execution.plan.thought_process {
            hasher.update(step.content.as_bytes());
        }
        for call in &execution.plan.intended_tool_calls {
            hasher.update(call.tool_name.as_bytes());
            hasher.update(call.arguments.as_bytes());
        }
        hasher.finalize()
    };

    // Check if any tool calls failed
    let failed_tools: Vec<_> = execution
        .tool_results
        .iter()
        .filter(|result| !result.success)
        .collect();

    let compliant = failed_tools.is_empty();
    let reason = if compliant {
        format!(
            "All {} tool calls passed per-tool compliance checks during execution",
            execution.tool_calls.len()
        )
    } else {
        let failed_ids: Vec<String> = failed_tools
            .iter()
            .map(|r| r.call_id.to_string())
            .collect();
        format!(
            "{} of {} tool calls failed: [{}]",
            failed_tools.len(),
            execution.tool_calls.len(),
            failed_ids.join(", ")
        )
    };

    ComplianceResult {
        compliant,
        reason,
        policy_hash: "per-tool-validation".to_string(),
        plan_hash: const_hex::encode(plan_hash.as_bytes()),
    }
}

/// Hash an agent execution for attestation
fn hash_execution(execution: &AgentExecution) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    // Hash session ID
    hasher.update(execution.session_id.as_bytes());

    // Hash plan
    hasher.update(execution.plan.system_prompt.as_bytes());
    hasher.update(execution.plan.user_query.as_bytes());

    for step in &execution.plan.thought_process {
        hasher.update(step.content.as_bytes());
    }

    // Hash tool calls
    for call in &execution.tool_calls {
        hasher.update(call.id.as_bytes());
        hasher.update(call.tool_name.as_bytes());
        hasher.update(call.arguments.as_bytes());
    }

    // Hash tool results
    for result in &execution.tool_results {
        hasher.update(result.call_id.as_bytes());
        hasher.update(&[result.success as u8]);
        hasher.update(result.result.as_bytes());
    }

    // Hash final response
    hasher.update(execution.final_response.as_bytes());

    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api::RouterRegister, types::SessionKeyPairs, utils::crypto};

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY
    async fn test_agent_query() {
        let session_key_pairs = SessionKeyPairs::default();

        let mut state = HypervisorState::default();
        state.set_session_key_pairs(session_key_pairs.clone());

        let server =
            axum_test::TestServer::new(Router::new().register_api(api_register).with_state(state))
                .unwrap();

        let sk = k256::ecdsa::SigningKey::random(&mut rand::rngs::OsRng);
        let user_pk = sk.verifying_key();

        let (session_pk, session_id) = session_key_pairs.create(user_pk);
        let cipher = crypto::create_encrypt_key(&sk, &session_pk, session_id).unwrap();

        let nonce = crypto::derive_msg_nonce(session_id);
        let query = "What is the current price of Bitcoin?";
        let encrypted_query = cipher.encrypt(&nonce, query.as_bytes()).unwrap();

        let response = server
            .post("/agent/query")
            .json(&AgentQueryRequest {
                encrypted_query: const_hex::encode(&encrypted_query),
                public_key: crypto::pk_to_hex(user_pk),
                use_llm_compliance: false,
            })
            .await;

        response.assert_status_ok();

        let result: AgentQueryResponse = response.json();

        let response_nonce = *aes_gcm_siv::Nonce::from_slice(
            &const_hex::decode(result.response_nonce).unwrap(),
        );

        let decrypted_response = cipher
            .decrypt(
                &response_nonce,
                const_hex::decode(&result.encrypted_response)
                    .unwrap()
                    .as_slice(),
            )
            .unwrap();

        let response_text = String::from_utf8(decrypted_response).unwrap();
        println!("Agent Response: {}", response_text);
        assert!(!response_text.is_empty());
    }
}
