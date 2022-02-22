#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqwmll8gpepveqmerjz78lppwpqf6kmfq0jg9qm8r04f"

TOKEN_TICK="0x424142414e45" # AXONE
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
