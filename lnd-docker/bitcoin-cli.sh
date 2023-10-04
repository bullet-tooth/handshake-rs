#!/usr/bin/env bash

docker exec -it bitcoind bitcoin-cli -rpcuser=user -rpcpassword=password -chain=regtest -rpcport=18443 "$@"
