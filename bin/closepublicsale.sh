#!/bin/bash

# Arguments: royalties jsoncid imagecid tag1,tag2 => encoded in HEX 
# https://www.online-toolz.com/tools/text-hex-convertor.php
# + add 0x before

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq68n2d9wce8jf7xh2jqs20y8f7707rnj4jg9qevfwmm"


erdpy --verbose \
    contract call ${SC_ADDRESS} \
    --recall-nonce \
    --pem="wallets/owner/owner.pem" \
    --gas-limit=100000000 \
    --function="pausePublicSale" \
    --send \
    --proxy="https://devnet-gateway.elrond.com" \
    --chain="D"
