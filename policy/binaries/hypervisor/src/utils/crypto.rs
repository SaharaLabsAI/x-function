use aes_gcm_siv::{Aes256GcmSiv, KeyInit, Nonce};
use anyhow::anyhow;
use k256::{
    ecdh::diffie_hellman,
    ecdsa::{SigningKey, VerifyingKey},
};
use secrecy::{ExposeSecret, ExposeSecretMut, SecretSlice};
use uuid::Uuid;

pub fn create_encrypt_key(
    sk: &SigningKey,
    pk: &VerifyingKey,
    session_id: Uuid,
) -> anyhow::Result<Aes256GcmSiv> {
    let shared_sk = diffie_hellman(sk.as_nonzero_scalar(), pk.as_affine());
    
    let hkdf = shared_sk.extract::<k256::sha2::Sha256>(Some(session_id.as_bytes()));

    let mut msg_key = SecretSlice::new(Box::new([0u8; 32]));
    hkdf.expand(&[], msg_key.expose_secret_mut())
        .map_err(|e| anyhow!(e.to_string()))?;

    let key = aes_gcm_siv::Key::<Aes256GcmSiv>::from_slice(msg_key.expose_secret());

    Ok(Aes256GcmSiv::new(key))
}

pub fn derive_msg_nonce(data: impl AsRef<[u8]>) -> Nonce {
    let hash: [u8; 32] = blake3::hash(data.as_ref()).into();

    Nonce::from_iter(hash[..12].iter().map(|u| *u))
}

pub fn pk_to_hex(pk: &VerifyingKey) -> String {
    pk.to_encoded_point(true).to_string()
}

pub fn pk_from_hex(pk_hex: &str) -> anyhow::Result<VerifyingKey> {
    let pk_bytes = const_hex::decode(pk_hex)?;

    let point = k256::EncodedPoint::from_bytes(pk_bytes)?;
    let pk = k256::ecdsa::VerifyingKey::from_encoded_point(&point)?;

    Ok(pk)
}
