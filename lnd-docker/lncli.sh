#!/usr/bin/env bash

docker exec -it lnd lncli --network=regtest "$@"
