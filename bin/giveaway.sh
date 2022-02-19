#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq24wsm2c3yr3y24gqa93zhvty3y49xaehjg9qfz77je"

HEIDI_ADDRESS_BECH32="erd1gmpleuv62ap74c8qs52w3qjpues8xdhk00nmdr0sdkmes4lchxzsfu0dwr"
HEIDI_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${HEIDI_ADDRESS_BECH32})

OWNER_PEM="wallets/owner/owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"

GAS_LIMIT=600000000

give() {
    [[ $1 == "giveaway" ]] && GAS_LIMIT=11100000 
       
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

# give giveaway 1
give giveawayMany 8
# give giveaway
