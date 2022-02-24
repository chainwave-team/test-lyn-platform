declare -a ADRESSES=(
    "carol"
    "alice"
    "bob"
    "dan"
    "eve"
    "mike"
    "grace"
    # "heidi"
    # "ivan"
    # "judy"
    # "wallet1"
    # "wallet2"
    # "wallet3"
    # "wallet4"
    # "wallet5"
    # "wallet6"
    # "wallet7"
    # "wallet8"
    # "wallet9"
    # "wallet10"
)

bulkTx() {
    for name in "${ADRESSES[@]}"; do
        ./bin/mint.sh $name
        sleep 3
    done

}

bulkTx
