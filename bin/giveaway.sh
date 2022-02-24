#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq6etagc8fufs8f85u4x8xt7cv7kklp8xfjg9qa054fd"

HEIDI_ADDRESS_BECH32="erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx"
HEIDI_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${HEIDI_ADDRESS_BECH32})

OWNER_PEM="wallets/owner/owner.pem"

# PROXY="https://devnet-gateway.elrond.com"
# CHAIN="D"
# # TESTNET CONFIG
PROXY="https://testnet-gateway.elrond.com"
CHAIN="T"
# # MAINNET CONFIG
# PROXY="https://gateway.elrond.com"
# CHAIN="M"

GAS_LIMIT=600000000

give() {
    [[ $1 == "giveaway" ]] && GAS_LIMIT=35000000
    erdpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --function $1 \
        --arguments ${HEIDI_ADDRESS_HEX} $2 \
        --send \
        --proxy=${PROXY} \
        --chain=${CHAIN}
}

# Run
# ---

give giveaway 10
# give giveawayMany 5
# give giveaway
