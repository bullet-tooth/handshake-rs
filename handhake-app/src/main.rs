use std::net::TcpStream;
use std::str::FromStr;
use std::{env, process};

use secp256k1::PublicKey;

use handshake_protocol::util;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let stream = TcpStream::connect(config.node_address).unwrap_or_else(|err| {
        eprintln!("Problem with connection: {}", err);
        process::exit(1);
    });

    let pk = PublicKey::from_str(config.node_pk.as_str()).unwrap_or_else(|err| {
        eprintln!("Problem parsing public key: {}", err);
        process::exit(1);
    });
    let sk = util::generate_random_keys().secret_key();

    let result = handshake_protocol::handshake(stream, sk, pk);
    match result {
        Ok(keys) => println!("Setup protocol keys: {:?}", keys),
        Err(e) => eprintln!("Error protocol setup {e}"),
    }
}

struct Config {
    node_address: String,
    node_pk: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let node_address = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the target node address"),
        };

        let node_pk = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the target node public key"),
        };

        Ok(Config {
            node_address,
            node_pk,
        })
    }
}
