#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqrjv97sjq5thmveexdwnlr8yfnkh62y9jjg9q402evd"

HEIDI_ADDRESS_BECH32=$1
HEIDI_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${HEIDI_ADDRESS_BECH32})

OWNER_PEM="wallets/owner/owner.pem"

# DEVNET CONFIG
# PROXY="https://devnet-gateway.elrond.com"
# CHAIN="D"
# # TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"
# # MAINNET CONFIG
PROXY="https://gateway.elrond.com"
CHAIN="M"

wl() {
    echo $1 ${HEIDI_ADDRESS_HEX}
    erdpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${OWNER_PEM} \
        --gas-limit=6000000 \
        --function whiteList \
        --arguments ${HEIDI_ADDRESS_HEX} $2 \
        --send \
        --proxy=${PROXY} \
        --chain=${CHAIN}
}

# Run
# ---
wl whiteList
