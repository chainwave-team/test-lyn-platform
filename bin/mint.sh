#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgql2ltnqk8ze4yfh8ukgrtdn3j3z284y24jg9qd5seek"

OWNER_PEM="wallets/owner/owner.pem"
HEIDI_PEM="wallets/users/bob.pem"

erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem=${HEIDI_PEM} \
    --gas-limit=30000000\
    --value=200000000000000000 \
    --function="mint" \
    --send \
    --proxy="https://devnet-gateway.elrond.com" \
    --chain="D"
