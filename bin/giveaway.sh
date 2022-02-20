#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqxjcdsc6r8pyy9qrftgju8rtwagz2vrgrjg9qhzqjl7"

HEIDI_ADDRESS_BECH32="erd15gaxx4zlwfq6zlzyuxnnmj9mntd8ee3ght8hl9yzvklypyxv3yrqdp8m7z"
HEIDI_ADDRESS_HEX=0x$(erdpy wallet bech32 --decode ${HEIDI_ADDRESS_BECH32})

OWNER_PEM="wallets/owner/owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"

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

# give giveaway 
# give giveawayMany 7
give giveaway
