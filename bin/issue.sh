#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqas9uuf68zhdmrmf3894vpfmt8dxw9rkt2yyqp4d8kt"

TOKEN_TICK="0x4c594e584545"     # LYNXEE
TOKEN_NAME="0x4c796e786565"     # Lynxee

# PROXY="https://devnet-gateway.elrond.com"
# CHAIN="D"
# # TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"

# # MAINNET CONFIG
PROXY="https://gateway.elrond.com"
CHAIN="1"

erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem="wallets/owner/neodium.pem" \
    --gas-limit=100000000 \
    --value=50000000000000000 \
    --function="issueToken" \
    --arguments ${TOKEN_NAME} ${TOKEN_TICK} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
