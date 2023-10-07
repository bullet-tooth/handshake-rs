use std::str::FromStr;

use secp256k1::{KeyPair, PublicKey, Secp256k1, SecretKey};

use handshake_protocol::acts::Act2;
use handshake_protocol::handshake_protocol::HandshakeProtocol;

#[test]
fn test_successful_handshake() {
    let ephemeral = KeyPair::from_secret_key(
        &Secp256k1::new(),
        &SecretKey::from_str("1212121212121212121212121212121212121212121212121212121212121212")
            .unwrap(),
    );

    let protocol = HandshakeProtocol::new(
        SecretKey::from_str("1111111111111111111111111111111111111111111111111111111111111111")
            .unwrap(),
        PublicKey::from_str("028d7500dd4c12685d1f568b4c2b5048e8534b873319f3a8daa612b469132ec7f7")
            .unwrap(),
    );
    let protocol = protocol.move_act1(Some(ephemeral)).unwrap();

    assert_eq!(hex::encode(protocol.act.to_vec()), "00036360e856310ce5d294e8be33fc807077dc56ac80d95d9cd4ddbd21325eff73f70df6086551151f58b8afe6c195782c6a");

    let mut act_two = [0_u8; 50];
    act_two.copy_from_slice(&hex::decode("0002466d7fcae563e5cb09a0d1870bb580344804617879a14949cf22285f1bae3f276e2470b93aac583c9ef6eafca3f730ae").unwrap());

    let protocol = protocol.receive_act2(Act2::from(act_two)).unwrap();

    let protocol = protocol.move_act3().unwrap();
    let act3 = protocol.act;

    assert_eq!(hex::encode(act3.to_vec()), "00b9e3a702e93e3a9948c2ed6e5fd7590a6e1c3a0344cfc9d5b57357049aa22355361aa02e55a8fc28fef5bd6d71ad0c38228dc68b1c466263b47fdf31e560e139ba");
}
