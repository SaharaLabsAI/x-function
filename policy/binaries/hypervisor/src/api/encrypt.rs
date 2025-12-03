use anyhow::Context;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::HypervisorError,
    types::HypervisorState,
    utils::{attest::generate_raw_report, crypto},
};

pub(crate) fn api_register(router: Router<HypervisorState>) -> Router<HypervisorState> {
    router
        .route("/encrypt/create_keypair", post(create_keypair))
        .route(
            "/verifiable/encrypt/create_keypair",
            post(verifiable_create_keypair),
        )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiableCreateKeyPairResponse {
    pub session_pubkey: String,
    pub session_id: Uuid,
    pub quote: String,
}

async fn verifiable_create_keypair(
    state: State<HypervisorState>,
    req: Json<CreateKeyPairRequest>,
) -> Result<Json<VerifiableCreateKeyPairResponse>, HypervisorError> {
    let Json(raw_resp) = create_keypair(state, req).await?;

    let session_pk = const_hex::decode(raw_resp.session_pubkey.as_str()).expect("impossible");
    let report = generate_raw_report(&[session_pk.as_slice(), raw_resp.session_id.as_bytes()]);

    let quote = attest::get_quote(report)
        .context("get create keypair quote")
        .context(StatusCode::INTERNAL_SERVER_ERROR)?;

    let verifiable_resp = VerifiableCreateKeyPairResponse {
        session_pubkey: raw_resp.session_pubkey,
        session_id: raw_resp.session_id,
        quote: const_hex::encode(quote.to_bytes()),
    };

    Ok(Json(verifiable_resp))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKeyPairRequest {
    pub pubkey: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKeyPairResponse {
    pub session_pubkey: String,
    pub session_id: Uuid,
}

/*
async fn create_keypair(
    State(state): State<HypervisorState>,
    Json(req): Json<CreateKeyPairRequest>,
) -> Result<Json<CreateKeyPairResponse>, HypervisorError> {
    let req_pk = crypto::pk_from_hex(&req.pubkey)
        .context("recover request pubkey")
        .context(StatusCode::BAD_REQUEST)?;

    let (session_pubkey, session_id) = state.create_session_keypair(&req_pk);

    let resp = CreateKeyPairResponse {
        session_pubkey: crypto::pk_to_hex(&session_pubkey),
        session_id,
    };

    Ok(Json(resp))
}
*/

async fn create_keypair(
    State(state): State<HypervisorState>,
    Json(req): Json<CreateKeyPairRequest>,
) -> Result<Json<CreateKeyPairResponse>, HypervisorError> {
    let req_pk = crypto::pk_from_hex(&req.pubkey)
        .map_err(|e| {
            eprintln!("DEBUG: pk_from_hex failed: {:?}", e);
            e
        })
        .context("recover request pubkey")
        .context(StatusCode::BAD_REQUEST)?;

    let (session_pubkey, session_id) = state.create_session_keypair(&req_pk);

    let resp = CreateKeyPairResponse {
        session_pubkey: crypto::pk_to_hex(&session_pubkey),
        session_id,
    };

    Ok(Json(resp))
}


#[cfg(test)]
mod tests {
    use crate::api::RouterRegister;

    use super::*;

    #[tokio::test]
    async fn test_api_create_keypair() {
        let server = axum_test::TestServer::new(
            Router::new()
                .register_api(api_register)
                .with_state(HypervisorState::default()),
        )
        .unwrap();

        let sk = k256::ecdsa::SigningKey::random(&mut rand::rngs::OsRng);
        let pk = sk.verifying_key().to_encoded_point(true).to_string();

        let response = server
            .post("/encrypt/create_keypair")
            .json(&CreateKeyPairRequest { pubkey: pk })
            .await;

        response.assert_status_ok();

        let resp = response.json::<CreateKeyPairResponse>();
        println!(
            "session pk {} session id {}",
            resp.session_pubkey, resp.session_id
        );
    }
}
