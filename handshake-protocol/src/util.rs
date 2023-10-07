use hkdf::Hkdf;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{KeyPair, Secp256k1, SecretKey};
use sha2::digest::{FixedOutput, Output};
use sha2::{Digest, Sha256};

pub(crate) fn generate_random_keys() -> KeyPair {
    let secp = Secp256k1::new();
    KeyPair::new(&secp, &mut OsRng)
}

pub(crate) fn hkdf(secret: &[u8], salt: &[u8]) -> (SecretKey, SecretKey) {
    let hkdf = Hkdf::<Sha256>::new(Some(salt), secret);
    let mut out = [0u8; 64];
    hkdf.expand(&[], &mut out).unwrap();

    (
        SecretKey::from_slice(&out[..32]).unwrap(),
        SecretKey::from_slice(&out[32..]).unwrap(),
    )
}

// TODO replace with macro
pub(crate) fn hash_roll(hash: &Output<Sha256>, data: impl AsRef<[u8]>) -> Output<Sha256> {
    Sha256::new_with_prefix(hash.as_slice())
        .chain_update(data)
        .finalize_fixed()
}

pub(crate) fn hash_roll_tag(
    hash: &Output<Sha256>,
    data: impl AsRef<[u8]>,
    tag: impl AsRef<[u8]>,
) -> Output<Sha256> {
    Sha256::new_with_prefix(hash.as_slice())
        .chain_update(data)
        .chain_update(tag)
        .finalize_fixed()
}
