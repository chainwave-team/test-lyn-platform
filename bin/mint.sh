#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq7cg98gd3sueje3vpagae9z97gde5nfdmjg9qek7fqu"

OWNER_PEM="wallets/owner/owner.pem"
# HEIDI_PEM="wallets/users/carol.pem"
HEIDI_PEM="wallets/users/$1.pem"

erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem=${HEIDI_PEM} \
    --gas-limit=60000000\
    --value=400000000000000000 \
    --function="mint" \
    --send \
    --proxy="https://devnet-gateway.elrond.com" \
    --chain="D"
