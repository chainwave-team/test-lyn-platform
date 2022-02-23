#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqf5kap4px390vm66j3huh54t3djq79acrjg9qyzgk4t"



#Uncomment for a single mint
# HEIDI_PEM="wallets/users/carol.pem"
#Uncomment for bulk mint
HEIDI_PEM="wallets/users/$1.pem"

erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem=${HEIDI_PEM} \
    --gas-limit=60000000 \
    --value=100000000000000000 \
    --function="mint" \
    --send \
    --proxy="https://devnet-gateway.elrond.com" \
    --chain="D"
