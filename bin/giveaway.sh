#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqas9uuf68zhdmrmf3894vpfmt8dxw9rkt2yyqp4d8kt"

HEIDI_ADDRESS_BECH32="erd1sh28vv8v4tvtqp8n7878hp43v73ka8pmzd7zcgrhdzp6ktgqj79q5ufrys"
HEIDI_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${HEIDI_ADDRESS_BECH32})

OWNER_PEM="wallets/owner/neodium.pem"

# PROXY="https://devnet-gateway.elrond.com"
# CHAIN="D"
# # TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"
# # MAINNET CONFIG
PROXY="https://gateway.elrond.com"
CHAIN="1"

GAS_LIMIT=600000000

give() {
    [[ $1 == "giveaway" ]] && GAS_LIMIT=11000000
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

# give giveaway 5
# give giveawayMany 50
# give giveaway 
