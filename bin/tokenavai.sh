#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqpyyefsnknpku3x7hpfyege6v5fvsn8wujg9qaju542"

HEIDI_ADDRESS_BECH32=$1
HEIDI_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${HEIDI_ADDRESS_BECH32})

OWNER_PEM="wallets/owner/owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"

wl() {
    erdpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${OWNER_PEM} \
        --gas-limit=60000000 \
        --function "tokensAvailable" \
        --send \
        --proxy=${PROXY} \
        --chain=${CHAIN}
}

wl