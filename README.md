# Handshake Implementation for Lightning Network

## Local Run LND Node

This repository contains scripts to run Lightning node locally
as a docker container within "regtest" bitcoin test network. It also runs `bitcoind` container that the Lightning node
connects to.

### Requirements

- docker
- shell, bash

### How to Run

1. Open a new terminal window and run (from the repo root):

```shell
cd lnd-docker
docker compose up
```

_It will start `lnd` and `bitcoind` nodes._

2. Open a second terminal window and run (from the repo root):

```shell
cd lnd-docker
./init-lnd.sh
```

_It will properly set up `lnd` node and print it's public kay to the console._

