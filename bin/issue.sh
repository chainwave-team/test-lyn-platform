#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqf5kap4px390vm66j3huh54t3djq79acrjg9qyzgk4t"

TOKEN_TICK="0x424142414e45" # AXONE
TOKEN_NAME="0x41414145"     # AXNE

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
    --value=50000000000000000 \
    --function="issueToken" \
    --arguments ${TOKEN_NAME} ${TOKEN_TICK} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
