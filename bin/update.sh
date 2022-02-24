SC_ADDRESS="erd1qqqqqqqqqqqqqpgqx7ev4dfunrkckv4ysemrvqmz5v8rd9nvjg9qky0mx8"
ROYALTIES="0x31303030"                                                                                                               # 1000 means 10%
JSON_CID="0x62616679626569657762667779326333337a7a726e367535377a36796d6e69346a697864736372796a376a6f767975696b6e736b6c667162346e34"  # bafybeiewbfwy2c33zzrn6u57z6ymni4jixdscryj7jovyuiknsklfqb4n4
IMAGE_CID="0x626166796265696466796734746b78617a637269683365616f6370776e346d363776796863756f6372756a77706c6536796a6f6c786b746e69716d" # bafybeidfyg4tkxazcrih3eaocpwn4m67vyhcuocrujwple6yjolxktniqm
TAGS="0x746167312c74616732"                                                                                                          # tag1,tag2

# DEVNET CONFIG
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
# TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"
# MAINNET CONFIG
# PROXY="https://gateway.elrond.com"
# CHAIN="M"

erdpy --verbose \
    contract upgrade ${SC_ADDRESS} \
    --pem="wallets/owner/owner.pem" \
    --project="." \
    --gas-limit=300000000 \
    --arguments 1000 ${TAGS} \
    --send \
    --outfile="deploy.interaction.json" \
    --recall-nonce \
    --proxy=${PROXY} \
    --chain=${CHAIN}
