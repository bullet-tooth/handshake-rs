use std::io::{Read, Write};
use std::net::TcpStream;

use secp256k1::{PublicKey, SecretKey};

use crate::acts::Act2;
use crate::error::Result;
use crate::handshake_protocol::{HandshakeProtocol, ProtocolKeys};

pub mod acts;
pub mod error;
pub mod handshake_protocol;
mod handshake_state;
mod nonce;
pub mod tag;
pub mod util;

pub fn handshake(
    mut stream: TcpStream,
    local_secret_key: SecretKey,
    remote_public_key: PublicKey,
) -> Result<ProtocolKeys> {
    let protocol = HandshakeProtocol::new(local_secret_key, remote_public_key);
    let protocol = protocol.move_act1(None)?;
    stream.write_all(&protocol.act)?;

    let mut act2 = Act2::default();
    stream.read_exact(&mut act2)?;
    let protocol = protocol.receive_act2(act2)?;

    let protocol = protocol.move_act3()?;
    stream.write_all(&protocol.act)?;

    Ok(protocol.keys)
}
