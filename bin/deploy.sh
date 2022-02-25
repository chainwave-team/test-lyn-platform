#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX
# https://www.online-toolz.com/tools/text-hex-convertor.php

TAGS="0x4c796e7865652c4e656f6469756d2c4e61747572652c4c796e782c416e696d616c73"                                                                                                          # tag1,tag2

# DEVNET CONFIG
# PROXY="https://devnet-gateway.elrond.com"
# CHAIN="D"

# TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"

# MAINNET CONFIG
PROXY="https://gateway.elrond.com"
CHAIN="1"

erdpy --verbose \
    contract deploy \
    --recall-nonce \
    --project="." \
    --pem="wallets/owner/neodium.pem" \
    --gas-limit=600000000 \
    --arguments 1000 ${TAGS} \
    --send \
    --outfile="deploy.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN}
