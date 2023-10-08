# Handshake Implementation for Lightning Network

This repository represents P2P Handshake protocol implementation in Rust for
[Lightning Network](https://lightning.network/) – layer 2 solution over Bitcoin blockchain.

Handshake protocol implements the [BOLT-08 specification](https://github.com/lightning/bolts/blob/master/08-transport.md)
that's based on `Noise_XK` authenticated key agreement performed in three sequential steps (acts).

At the end of the successful handshake each node gets the following keys:

- `sk` is the key to be used to encrypt messages for sending to the target node
- `rk` is the key to be used to decrypt messages sent by the target node

## Implementation Details

There are the following packages:

- [handshake-protocol](handshake-protocol) – library with the handshake protocol implementation 
- [handshake-app](handshake-app) – command line application to run handshake

**NB:** the current implementation has significant limitation and can perform as a "initiator" only.

## Build and Run

### Requirements

- Rust
- docker
- shell, bash

### How to Build

To build the project simply run in a terminal (from the project root):

```shell
cargo build
```

### How to Run

To run the command line application run in a terminal (from the project root):

```shell
cargo run --bin handshake-app $target_node $target_node_pk 
```

Where,

- `$target_node` is a target node address, for example `localhost:9735`
- `$target_node_pk` is a target node public key, for example `032f5cf5c8c7d13c526731c0848eaaef94fab72b11a5b785fb0fe8a86eed324907`

## Local Run with LND Node

This repository contains scripts to run Lightning node locally as a docker container within "regtest" 
bitcoin test network. 
Such as Lightning is a layer 2 solution it requires `bitcoind` container that the Lightning node connects to.
Also, it requires some initial setup to make the node alive which is performed by `lnd-docker/init-lnd.sh` script (see below).  

1. Open a new terminal window and run (from the repo root):

```shell
cd lnd-docker
docker compose up
```

It will start `lnd` and `bitcoind` nodes.

2. Open a second terminal window and run (from the repo root):

```shell
cd lnd-docker
./init-lnd.sh
```

It will properly set up `lnd` node and it will print it's public kay to the console with the message like:

```
lnd successfully initialized!
==========================================

Target LND node public key: 032f5cf5c8c7d13c526731c0848eaaef94fab72b11a5b785fb0fe8a86eed324907

==========================================

```

3. Open a third terminal window and run the handshake application specifying the provided key (from the repo root):

```shell
cargo run --bin handshake-app localhost:9735 032f5cf5c8c7d13c526731c0848eaaef94fab72b11a5b785fb0fe8a86eed324907
```

At the successful handshake it should output into console the protocol keys, for example:
```
Setup protocol keys: ProtocolKeys (sk: de1573057cf19104373a198b175ed3662b423a694af081307b28910614cdba8d, rk: a197b9c8b90fab6e7d0746b23585801cd5dfeb584b4bce19e0ea86dfd48feb29)
```

Also, you should see in the lightning node logs messages describing a successful peer connection, similar to the following:
```
lnd       | 2023-10-08 15:04:53.602 [INF] SRVR: New inbound connection from 192.168.65.1:50591
lnd       | 2023-10-08 15:04:53.602 [INF] SRVR: Finalizing connection to 02c6c566039545fa9be42b01ee5d3a568dabc72211cc1248ed056a90bd3c684ab3@192.168.65.1:50591, inbound=true
```

## TODOs

- Add "receiver" support to make the protocol bi-directional
- Improve Rust docs
- Improve test coverage
