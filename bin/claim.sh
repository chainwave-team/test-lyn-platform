#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqa2nz4zk8h8d2k7ynrvg6hhl2h40txsj2jg9qqsdnwr"

OWNER_PEM="wallets/owner/owner.pem"

# DEVNET CONFIG
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
# # TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"
# # MAINNET CONFIG
# PROXY="https://gateway.elrond.com"
# CHAIN="M"

PAYLOAD=""
claim() {
    erdpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${OWNER_PEM} \
        --gas-limit=60000000 \
        --function ClaimDeveloperRewards \
        --send \
        --proxy=${PROXY} \
        --chain=${CHAIN}
}

# Run
# ---
claim
