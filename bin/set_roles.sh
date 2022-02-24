#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqf5kap4px390vm66j3huh54t3djq79acrjg9qyzgk4t"

# DEVNET CONFIG
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
# # TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"
# # MAINNET CONFIG
# PROXY="https://gateway.elrond.com"
# CHAIN="M"
erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem="wallets/owner/owner.pem" \
    --gas-limit=100000000 \
    --function="setLocalRoles" \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
