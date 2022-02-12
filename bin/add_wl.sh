#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq2avw32d34da7hy0e4n0kjxe3afe7g4kjjg9qhfm055"

HEIDI_ADDRESS_BECH32="erd1gmpleuv62ap74c8qs52w3qjpues8xdhk00nmdr0sdkmes4lchxzsfu0dwr"
HEIDI_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${HEIDI_ADDRESS_BECH32})

OWNER_PEM="wallets/owner/owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"

wl() {
    erdpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${OWNER_PEM} \
        --gas-limit=100000000 \
        --function $1 \
        --arguments ${HEIDI_ADDRESS_HEX} $2 \
        --send \
        --proxy=${PROXY} \
        --chain=${CHAIN}
}

# Run
# ---

# give giveaway
# give giveawayMany 3
wl whiteList