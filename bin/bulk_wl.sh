declare -a ADRESSES=(
    "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
    "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx"
    "erd1dc3yzxxeq69wvf583gw0h67td226gu2ahpk3k50qdgzzym8npltq7ndgha"
    "erd1k2s324ww2g0yj38qn2ch2jwctdy8mnfxep94q9arncc6xecg3xaq6mjse8"
    "erd18tudnj2z8vjh0339yu3vrkgzz2jpz8mjq0uhgnmklnap6z33qqeszq2yn4"
    "erd1kyaqzaprcdnv4luvanah0gfxzzsnpaygsy6pytrexll2urtd05ts9vegu7"
    "erd1r69gk66fmedhhcg24g2c5kn2f2a5k4kvpr6jfw67dn2lyydd8cfswy6ede"
    "erd13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7shalqrx"
    "erd1fggp5ru0jhcjrp5rjqyqrnvhr3sz3v2e0fm3ktknvlg7mcyan54qzccnan"
    "erd1kdl46yctawygtwg2k462307dmz2v55c605737dp3zkxh04sct7asqylhyv"
    "erd1z32fx8l6wk9tx4j555sxk28fm0clhr0cl88dpyam9zr7kw0hu7hsx2j524"
    "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa"
)
#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq6etagc8fufs8f85u4x8xt7cv7kklp8xfjg9qa054fd"

OWNER_PEM="wallets/owner/owner.pem"

# DEVNET CONFIG
# PROXY="https://devnet-gateway.elrond.com"
# CHAIN="D"
# # TESTNET CONFIG
PROXY="https://testnet-gateway.elrond.com"
CHAIN="T"
# # MAINNET CONFIG
# PROXY="https://gateway.elrond.com"
# CHAIN="M"

PAYLOAD=""
wl() {
    for adress in "${ADRESSES[@]}"; do
        hex=$(erdpy wallet bech32 --decode $adress)
        PAYLOAD+=$hex
    done
    echo $PAYLOAD
    erdpy --verbose contract call ${SC_ADDRESS} \
        --recall-nonce \
        --pem=${OWNER_PEM} \
        --gas-limit=60000000 \
        --function whiteListMany@${PAYLOAD} \
        --send \
        --proxy=${PROXY} \
        --chain=${CHAIN}
}

# Run
# ---
wl
