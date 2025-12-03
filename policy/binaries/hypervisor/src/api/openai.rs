use aes_gcm_siv::aead::Aead;
use anyhow::{anyhow, Context};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};
use uuid::Uuid;

use crate::{
    error::HypervisorError,
    types::HypervisorState,
    utils::{self, commitment_openai, crypto},
};

pub(crate) fn api_register(router: Router<HypervisorState>) -> Router<HypervisorState> {
    router
        .route("/openai/query", post(query_openai))
        .route("/verifiable/openai/query", post(verifiable_query_openai))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIQueryRequest {
    /// Encrypted prompt (hex-encoded)
    pub encrypted_prompt: String,
    /// User's public key (hex-encoded compressed SECP256K1 public key)
    pub public_key: String,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIQueryResponse {
    pub session_id: Uuid,
    /// Encrypted response (hex-encoded)
    pub encrypted_response: String,
    /// Nonce used for response encryption (hex-encoded)
    pub response_nonce: String,
    /// Model used
    pub model: String,
    /// Commitment to the query (prompt + response + metadata)
    pub query_commitment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiableOpenAIQueryResponse {
    pub session_id: Uuid,
    /// Encrypted response (hex-encoded)
    pub encrypted_response: String,
    /// Nonce used for response encryption (hex-encoded)
    pub response_nonce: String,
    /// Model used
    pub model: String,
    /// Commitment to the query (prompt + response + metadata)
    pub query_commitment: String,
    /// TEE attestation quote (hex-encoded)
    pub quote: String,
}

async fn verifiable_query_openai(
    state: State<HypervisorState>,
    req: Json<OpenAIQueryRequest>,
) -> Result<Json<VerifiableOpenAIQueryResponse>, HypervisorError> {
    let Json(resp) = query_openai(state, req).await?;
    
    let commitment: [u8; 32] =
        const_hex::decode_to_array(&resp.query_commitment).expect("impossible");

    let quote = attest::get_quote(utils::attest::generate_raw_report_from_hash(commitment))
        .context("get openai query quote")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    let verifiable_resp = VerifiableOpenAIQueryResponse {
        session_id: resp.session_id,
        encrypted_response: resp.encrypted_response,
        response_nonce: resp.response_nonce,
        model: resp.model,
        query_commitment: resp.query_commitment,
        quote: const_hex::encode(quote.to_bytes()),
    };

    Ok(Json(verifiable_resp))
}

#[tracing::instrument(skip(state, req), err)]
async fn query_openai(
    State(state): State<HypervisorState>,
    Json(req): Json<OpenAIQueryRequest>,
) -> Result<Json<OpenAIQueryResponse>, HypervisorError> {
    // Validate request
    validate_query_request(&req)?;

    let start_time = std::time::Instant::now();

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

    // Decrypt the prompt
    let decrypted_prompt = {
        let encrypted_bytes = const_hex::decode(&req.encrypted_prompt)
            .context(StatusCode::BAD_REQUEST)
            .context("invalid prompt hex")?;

        debug!(
            session_id = %session_id,
            public_key = %req.public_key,
            encrypted_len = encrypted_bytes.len(),
            nonce = %const_hex::encode(msg_nonce.as_slice()),
            "attempting to decrypt prompt"
        );

        let decrypted = cipher
            .decrypt(&msg_nonce, encrypted_bytes.as_slice())
            .map_err(|e| {
                debug!(
                    session_id = %session_id,
                    error = %e,
                    "decryption failed"
                );
                anyhow!(e.to_string())
            })
            .context(StatusCode::BAD_REQUEST)
            .context("decrypt prompt")?;

        String::from_utf8(decrypted)
            .context(StatusCode::BAD_REQUEST)
            .context("prompt isn't valid UTF-8")?
    };

    info!(
        session_id = %session_id,
        public_key = req.public_key,
        prompt_length = decrypted_prompt.len(),
        "processing OpenAI query request"
    );

    // Get OpenAI API key from environment
    let api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY not set")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Build OpenAI API request
    let client = reqwest::Client::new();
    let request_body = serde_json::json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "user",
                "content": decrypted_prompt
            }
        ],
        "temperature": req.temperature.unwrap_or(0.7),
        "max_tokens": req.max_tokens.unwrap_or(1000)
    });

    // Call OpenAI API
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("failed to send request to OpenAI")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow!("OpenAI API error {}: {}", status, error_text))
            .context(StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Parse OpenAI response
    let openai_response: serde_json::Value = response
        .json()
        .await
        .context("failed to parse OpenAI response")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    let response_text = openai_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow!("Invalid OpenAI response format"))
        .context(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let model = openai_response["model"]
        .as_str()
        .unwrap_or("gpt-4o")
        .to_string();

    info!(
        session_id = %session_id,
        public_key = req.public_key,
        execution_time_ms = start_time.elapsed().as_millis(),
        response_length = response_text.len(),
        status = "success",
        msg = "OpenAI query completed successfully"
    );

    // Encrypt the response
    let response_nonce = crypto::derive_msg_nonce(response_text.as_bytes());
    let encrypted_response = {
        let encrypted = cipher
            .encrypt(&response_nonce, response_text.as_bytes())
            .map_err(|e| anyhow!(e.to_string()))
            .context("encrypt response")
            .context(StatusCode::INTERNAL_SERVER_ERROR)?;

        const_hex::encode(encrypted)
    };

    // Build commitment: hash(user_pk, session_pk, session_id, encrypted_prompt, model, response_nonce, encrypted_response)
    let query_commitment = commitment_openai::build_query_commitment(
        &user_pk,
        session_sk.verifying_key(),
        session_id,
        &req.encrypted_prompt,
        &model,
        req.temperature.unwrap_or(0.7),
        req.max_tokens.unwrap_or(1000),
        response_nonce,
        &encrypted_response,
    );

    let resp = OpenAIQueryResponse {
        session_id,
        encrypted_response,
        response_nonce: const_hex::encode(response_nonce),
        model,
        query_commitment: const_hex::encode(query_commitment),
    };

    Ok(Json(resp))
}

/// Validate query request
fn validate_query_request(request: &OpenAIQueryRequest) -> Result<(), HypervisorError> {
    let validate = || -> anyhow::Result<()> {
        // Validate encrypted_prompt
        anyhow::ensure!(
            !request.encrypted_prompt.trim().is_empty(),
            "encrypted_prompt cannot be empty"
        );

        // Validate public_key
        anyhow::ensure!(
            !request.public_key.trim().is_empty(),
            "public_key cannot be empty"
        );

        Ok(())
    };

    validate().context(StatusCode::BAD_REQUEST)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use aes_gcm_siv::aead::Aead;

    use crate::utils::crypto;
    use crate::{api::RouterRegister, types::SessionKeyPairs};

    use super::*;

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY
    async fn test_api_query_openai() {
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
        let prompt = "What is 2+2? Answer with just the number.";
        let encrypted_prompt = cipher.encrypt(&nonce, prompt.as_bytes()).unwrap();

        let response = server
            .post("/openai/query")
            .json(&OpenAIQueryRequest {
                encrypted_prompt: const_hex::encode(&encrypted_prompt),
                public_key: crypto::pk_to_hex(user_pk),
                temperature: Some(0.0),
                max_tokens: Some(50),
            })
            .await;

        response.assert_status_ok();

        let result: OpenAIQueryResponse = response.json();
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
        println!("Response: {}", response_text);
        assert!(response_text.contains("4"));
    }

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY
    async fn test_verifiable_query_openai() {
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
        let prompt = "Explain quantum computing in one sentence.";
        let encrypted_prompt = cipher.encrypt(&nonce, prompt.as_bytes()).unwrap();

        let response = server
            .post("/verifiable/openai/query")
            .json(&OpenAIQueryRequest {
                encrypted_prompt: const_hex::encode(&encrypted_prompt),
                public_key: crypto::pk_to_hex(user_pk),
                temperature: Some(0.7),
                max_tokens: Some(100),
            })
            .await;

        response.assert_status_ok();

        let result: VerifiableOpenAIQueryResponse = response.json();
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
        println!("Response: {}", response_text);
        println!("Quote: {}", result.quote);
        assert!(!result.quote.is_empty());
    }
}
