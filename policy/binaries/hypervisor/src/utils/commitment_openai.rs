use aes_gcm_siv::Nonce;
use k256::ecdsa::VerifyingKey;
use uuid::Uuid;

use crate::utils;

/// Build commitment for OpenAI query
/// Commitment = hash(user_pk, session_pk, session_id, encrypted_prompt, model, temperature, max_tokens, response_nonce, encrypted_response)
pub fn build_query_commitment(
    user_pk: &VerifyingKey,
    session_pk: &VerifyingKey,
    session_id: Uuid,
    encrypted_prompt: &str,
    model: &str,
    temperature: f32,
    max_tokens: u32,
    response_nonce: Nonce,
    encrypted_response: &str,
) -> [u8; 32] {
    let entries = vec![
        user_pk.to_encoded_point(true).to_bytes(),
        session_pk.to_encoded_point(true).to_bytes(),
        Box::new(*session_id.as_bytes()),
        encrypted_prompt.as_bytes().into(),
        model.as_bytes().into(),
        temperature.to_le_bytes().to_vec().into(),
        max_tokens.to_le_bytes().to_vec().into(),
        response_nonce.to_vec().into(),
        encrypted_response.as_bytes().into(),
    ];

    utils::hasher::hash_multi(&entries)
}
