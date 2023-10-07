use secp256k1::ecdh::SharedSecret;
use secp256k1::{KeyPair, PublicKey, Secp256k1, SecretKey};
use std::fmt;

use crate::acts::{Act1, Act2, Act3};
use crate::error::Error::BadVersion;
use crate::handshake_state::HandshakeState;
use crate::nonce::Nonce;
use crate::tag::Tag;
use crate::util::{generate_random_keys, hash_roll, hash_roll_tag, hkdf};
use crate::Result;

pub const VERSION: u8 = 0;

pub struct HandshakeProtocol {
    state: HandshakeState,
}

pub struct HandshakeAct1 {
    state: HandshakeState,
    ephemeral: KeyPair,
    pub act: Act1,
}

pub struct HandshakeAct2 {
    state: HandshakeState,
    temp_k2: SecretKey,
    act: Act2,
}

pub struct HandshakeAct3 {
    pub act: Act3,
    pub keys: ProtocolKeys,
}

pub struct ProtocolKeys {
    pub sk: SecretKey,
    pub rk: SecretKey,
}

impl fmt::Debug for ProtocolKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ProtocolKeys (sk: {}, rk: {})",
            hex::encode(self.sk.as_ref()),
            hex::encode(self.rk.as_ref())
        )
    }
}

impl HandshakeProtocol {
    pub fn new(local_secret_key: SecretKey, remote_public_key: PublicKey) -> Self {
        let state = HandshakeState::new(local_secret_key, remote_public_key);
        HandshakeProtocol { state }
    }

    pub fn move_act1(&self, ephemeral: Option<KeyPair>) -> Result<HandshakeAct1> {
        let ephemeral = ephemeral.unwrap_or(generate_random_keys());

        let hash = hash_roll(&self.state.hash, ephemeral.public_key().serialize());

        let es = SharedSecret::new(&self.state.static_remote_key, &ephemeral.secret_key());

        let (ck, temp_k1) = hkdf(es.as_ref(), self.state.chaining_key.as_ref());

        let mut cipher_text = vec![];
        let c = chacha20_poly1305_aead::encrypt(
            temp_k1.as_ref(),
            &Nonce::from(0),
            hash.as_slice(),
            &[],
            &mut cipher_text,
        )?;

        let hash = hash_roll(&hash, c.as_ref());

        let state = self.state.chain_update(ck, hash);
        let act = Act1::new(VERSION, ephemeral.public_key(), Tag::from(c));

        Ok(HandshakeAct1 {
            state,
            ephemeral,
            act,
        })
    }
}

impl HandshakeAct1 {
    pub fn receive_act2(&self, act: Act2) -> Result<HandshakeAct2> {
        if VERSION != act.version() {
            return Err(BadVersion(act.version()));
        }

        let re = act.public_key();
        let c = act.tag();

        let hash = hash_roll(&self.state.hash, re.serialize());

        let ee = SharedSecret::new(&re, &self.ephemeral.secret_key());

        let (ck, temp_k2) = hkdf(ee.as_ref(), self.state.chaining_key.as_ref());

        let mut plain_text = vec![];
        chacha20_poly1305_aead::decrypt(
            temp_k2.as_ref(),
            &Nonce::from(0),
            hash.as_slice(),
            &[],
            &c,
            &mut plain_text,
        )?;

        let hash = hash_roll(&hash, c);

        Ok(HandshakeAct2 {
            state: self.state.chain_update(ck, hash),
            temp_k2,
            act,
        })
    }
}

impl HandshakeAct2 {
    pub fn move_act3(&self) -> Result<HandshakeAct3> {
        let initiator_pk = self.state.static_local_key.public_key(&Secp256k1::new());
        let hash = self.state.hash;

        let mut cipher_text = vec![];
        let c = chacha20_poly1305_aead::encrypt(
            self.temp_k2.as_ref(),
            &Nonce::from(1),
            hash.as_slice(),
            &initiator_pk.serialize(),
            &mut cipher_text,
        )?;
        let hash = hash_roll_tag(&hash, &cipher_text, c);
        let se = SharedSecret::new(&self.act.public_key(), &self.state.static_local_key);

        let (ck, temp_k3) = hkdf(se.as_ref(), self.state.chaining_key.as_ref());

        let mut cipher_text_t = vec![];
        let t = chacha20_poly1305_aead::encrypt(
            temp_k3.as_ref(),
            &Nonce::from(0),
            hash.as_slice(),
            &[],
            &mut cipher_text_t,
        )?;

        let (sk, rk) = hkdf(&[], ck.as_ref());

        let act = Act3::new(VERSION, cipher_text, Tag::from(c), Tag::from(t));

        Ok(HandshakeAct3 {
            act,
            keys: ProtocolKeys { sk, rk },
        })
    }
}
