use std::sync::Arc;

use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    EncodedPoint,
};
use uuid::Uuid;

use crate::Config;

#[derive(Clone, Default)]
pub(crate) struct HypervisorState {
    pub config: Config,
    session_key_pairs: SessionKeyPairs,
}

impl HypervisorState {
    pub fn new(config: Config) -> Self {
        HypervisorState {
            config,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn set_session_key_pairs(&mut self, session_key_pairs: SessionKeyPairs) {
        self.session_key_pairs = session_key_pairs;
    }

    pub fn create_session_keypair(self, pubkey: &VerifyingKey) -> (VerifyingKey, Uuid) {
        self.session_key_pairs.create(pubkey)
    }

    pub fn get_session_keypair(self, pubkey: &VerifyingKey) -> Option<(SigningKey, Uuid)> {
        self.session_key_pairs
            .0
            .get(&pubkey.to_encoded_point(true))
            .map(|i| i.to_owned())
    }
}

pub(crate) struct ServerContext {
    pub state: HypervisorState,
}

#[derive(Clone, Default)]
pub(crate) struct SessionKeyPairs(Arc<dashmap::DashMap<EncodedPoint, (SigningKey, Uuid)>>);

impl SessionKeyPairs {
    pub fn create(self, pubkey: &VerifyingKey) -> (VerifyingKey, Uuid) {
        let sk = k256::ecdsa::SigningKey::random(&mut rand::rngs::OsRng);
        let pk = sk.verifying_key().to_owned();
        let uuid = Uuid::now_v7();

        self.0.insert(pubkey.to_encoded_point(true), (sk, uuid));

        (pk, uuid)
    }
}
