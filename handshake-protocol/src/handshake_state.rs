use secp256k1::{PublicKey, SecretKey};

use sha2::digest::Output;
use sha2::{Digest, Sha256};

use crate::util::hash_roll;

const PROTOCOL_NAME: &str = "Noise_XK_secp256k1_ChaChaPoly_SHA256";
const PROLOGUE: &str = "lightning";

pub(crate) struct HandshakeState {
    pub(crate) static_local_key: SecretKey,
    pub(crate) static_remote_key: PublicKey,
    pub(crate) chaining_key: SecretKey,
    pub(crate) hash: Output<Sha256>,
}

impl HandshakeState {
    pub(crate) fn new(static_local_key: SecretKey, static_remote_key: PublicKey) -> Self {
        let hash: Output<Sha256> = Sha256::digest(PROTOCOL_NAME.as_bytes());

        let chaining_key = SecretKey::from_slice(hash.as_slice()).unwrap();

        let hash = hash_roll(&hash, PROLOGUE.as_bytes());
        let hash = hash_roll(&hash, static_remote_key.serialize());

        HandshakeState {
            chaining_key,
            hash,
            static_local_key,
            static_remote_key,
        }
    }

    pub(crate) fn chain_update(&self, chaining_key: SecretKey, hash: Output<Sha256>) -> Self {
        HandshakeState {
            chaining_key,
            hash,
            static_local_key: self.static_local_key,
            static_remote_key: self.static_remote_key,
        }
    }
}
