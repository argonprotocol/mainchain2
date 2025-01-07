#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e
BASEDIR=$(dirname "$0")/..

./target/release/argon-node build-spec --chain gen-testnet --raw > $BASEDIR/node/src/chain_spec/testnet1.json
