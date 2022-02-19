#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq8d0e9cvsr2lf4vzeej9hp6wd2usgju43jg9q43ev3z"

OWNER_PEM="wallets/owner/owner.pem"
HEIDI_PEM="wallets/users/carol.pem"

erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem=${HEIDI_PEM} \
    --gas-limit=600000000\
    --value=300000000000000000 \
    --function="mint" \
    --send \
    --proxy="https://devnet-gateway.elrond.com" \
    --chain="D"
