#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgql2ltnqk8ze4yfh8ukgrtdn3j3z284y24jg9qd5seek"

TOKEN_TICK="0x424158414e45" # AXONE
TOKEN_NAME="0x41414145" # AXNE

erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem="wallets/owner/owner.pem" \
    --gas-limit=100000000 \
    --value=50000000000000000 \
    --function="issueToken" \
    --arguments ${TOKEN_NAME} ${TOKEN_TICK} \
    --send \
    --proxy="https://devnet-gateway.elrond.com" \
    --chain="D"
