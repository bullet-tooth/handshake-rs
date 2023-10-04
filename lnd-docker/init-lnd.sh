#!/usr/bin/env bash

# Generate blocks to activate CTV, CLTV, SegWit.
#`generatetoaddress` requires some address so here is some random address
RANDOM_ADDRESS=bcrt1qgwev460zqprwlvnv45nq3tyuwgj4t8ukx8qs53
./bitcoin-cli.sh generatetoaddress 1000 $RANDOM_ADDRESS

LND_PASSWORD=12345678
./lncli-create.expect $LND_PASSWORD

sleep 2

LND_PUBKEY=$(./lncli.sh getinfo | jq -r ".identity_pubkey")

echo "=========================================="
echo ""
echo "Target LND node public key: $LND_PUBKEY"
echo ""
echo "=========================================="
echo ""
echo "Init done! Don't forget to stop the containers by running 'docker compose down'"
