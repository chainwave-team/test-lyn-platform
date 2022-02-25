#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqas9uuf68zhdmrmf3894vpfmt8dxw9rkt2yyqp4d8kt"

# DEVNET CONFIG
# PROXY="https://devnet-gateway.elrond.com"
# CHAIN="D"
# TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"
# # MAINNET CONFIG
PROXY="https://gateway.elrond.com"
CHAIN="1"


erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem="wallets/owner/neodium.pem" \
    --gas-limit=100000000 \
    --function="setLocalRoles" \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
