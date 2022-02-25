#// 20 max
declare -a ADRESSES=(
   "erd1e849tua3y07xvsgaswr0hzu9q23c6ks9q4r9rdvdq2ptfrf9t5ks0atm85"
   "erd1rasalv0gfhpc5cua3g667y098npjygc2u5z2wa9aptumfzv6d6qqq9zwy8"
   "erd13nwn7ys9mds6ttfk2suhvcmxcu3ve2tkkcr4g2g9c0n55j5g4m8q82c4hu"
   "erd1ek38zlzreqa6afuwrhwvfg64u7ldw8myfn3m3drqwlgh2432eessph95ut"
   "erd1lura3q5g9r0nj49l9punqv3552rrp7xakr2ha4p06hz89r039plqj0ymjm"
   "erd12pzauvar8sq0jyset305j0vjfhyytwzfa5720kzummp49udfgpfsvygj9k"
   "erd1lwmx0p7uk0lrp63dg60q8vtwyp64fdxk5y9htnvvjlags38dk6lq7avmqy"
   "erd1eqcva2vdugqclw43vg86vyju44av7gf2a9p9japqt480mnkfslys0cyhw2"
   "erd180r8xaqny0wycfl0tehhcmjh9cwh2n67w2e0uyhn36gsz23rfwqs7qhg40"
   "erd1sy9e0tet7aqw5mk5dyn4nn7d5atfeaclm7jspsslhn7gxut24mes73mwte"
   "erd1s45wnqf5zw2ns62frx9fzh2znzcrcxtvlur7d8sjshkcr7em8clq64v2t9"
   "erd1yjsendg4xu830y70rhanmlc3w0k6gjm8ehyy3vpv6u9kdeujdh2shz4h5c"
   "erd1rrhne4s9zf0wqwdtn4rtx2n2kl0fh26a6x6n4qhprv0s2twp99msfjeaav"
   "erd1pfs8xwfaypzyc8sm3dyufxamawhj7se68zdpw5njw9ppwgz7qfdqept4zn"
   "erd1w5u2fc80tye6v4au70cfetnpj0wgkznm4v7uwqkdgwxpqwl7zqaq26vzn5"
   "erd14dhg45peqmz6w38q969s8wy53f22fpahvkhcgfpcqlmnenc0na8quh92ju"
   "erd1w2dzy6su8k44pfqyvrskhajmyq9au4a0axuntvnmkpwf9pqgprws6mjxhw"
   "erd1ez0mykqpaeu0csjjre0al356pywmck4hc67tlhx73lqywrarg5vsnvhlem"
   "erd136unnt0dtpezfjpuc77xhv3qrlaw46mlzx69m0xsq2p3dwrgt8gqr2utke"
)
#!/bin/bash

# update me each time you have re-deployed
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqa2nz4zk8h8d2k7ynrvg6hhl2h40txsj2jg9qqsdnwr"

OWNER_PEM="wallets/owner/owner.pem"

# DEVNET CONFIG
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
# # TESTNET CONFIG
# PROXY="https://testnet-gateway.elrond.com"
# CHAIN="T"
# # MAINNET CONFIG
# PROXY="https://gateway.elrond.com"
# CHAIN="M"

PAYLOAD=""
wl() {
    for adress in "${ADRESSES[@]}"; do
        hex=$(erdpy wallet bech32 --decode $adress)
        PAYLOAD+=$hex
    done
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
