use secp256k1::{PublicKey, SecretKey};
use std::net::TcpStream;
use std::str::FromStr;

fn main() {
    let stream = TcpStream::connect("localhost:9735").unwrap();
    let pk =
        PublicKey::from_str("032f5cf5c8c7d13c526731c0848eaaef94fab72b11a5b785fb0fe8a86eed324907")
            .unwrap();
    let sk =
        SecretKey::from_str("1111111111111111111111111111111111111111111111111111111111111111")
            .unwrap();

    let result = handshake_protocol::handshake(stream, sk, pk);

    match result {
        Ok(keys) => println!("Setup protocol keys: {:?}", keys),
        Err(e) => eprintln!("Error protocol setup {e}"),
    }
}
