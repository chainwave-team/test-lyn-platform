# Lynxee NFT Collection

Rust based smart-contract for the #NFT Lynxee project on Elrond's network

## Install

### Requirements

- erdpy official erlong cli app
- rust

### Scripts

```bash
# compile contract
erdpy contract build

# run mandos tests
erdpy contract test
```

Note: Sometime when you are updating the contract interface, the build failed without rust error. Just remove the meta crate cache and re-build.

```sh
rm -rf ./meta/target
```

### IPFS test collection

- metadata: [`bafybeia4lqhl7g7wdd65ye76rw2s25ladr7khixevqeyjnao6azpwkjr2m`](https://bafybeia4lqhl7g7wdd65ye76rw2s25ladr7khixevqeyjnao6azpwkjr2m.ipfs.dweb.link/)
- images: [`bafybeiejtr7dewbts7uqrffwso4c6pvpkceiqqwninpv3qqkizlaqzaftu`](https://bafybeiejtr7dewbts7uqrffwso4c6pvpkceiqqwninpv3qqkizlaqzaftu.ipfs.dweb.link/)

## Flow

First steps - setup the collection - owner only

1. Build the rust contract to wasm.
2. Deploy the smart-contract (SC) on the blockchain. It calls the `init(...)`. Here, we inject project settings.
3. Issue the token, set the name, the ticker.
4. Set roles (to be able to create tokens).
5. (optional) pre-mint for the team and for the giveaways

At this step, the collection is almost ready (the mint requires to be activated), we can start the mint journey.
Everyone can mint, it costs EGLD, this action will be on our dapp (website).

6. Activate the mint journey by calling `startMinting` endpoint.
7. Pay, mint and transfer a token.
8. At the end of the 3000 items, close the mint function.
9. (question) The transfer, auctions, erc-721 interface (...) methods must be implemented in the current SC or are performed by the marketplaces or the elrond API?

## Structure

List of the methods and storage. The checkbox checked indicate that the feature is tested using mandos.

Note: Using RipGrep (grep rewritten in rust), you can quickly generate the API

```sh
# make it executable (may require "sudo" before)
chmod +x ./bin/generate_api.sh

# Generate API
./bin/generate_api.sh
```

### Endpoints

- `issueToken()`
- `setLocalRoles()`
- `setRoyalties()`
- `pauseMinting()`
- `startMinting()`
- `mint()`

### View

- `getMintPrice()`
- `getTokenId()`
- `getTokenName()`
- `getJsonCid()`
- `getImageCid()`

### Storage

- `tokenId`
- `tokenName`
- `jsonCid`
- `imageCid`
- `tags`
- `mintedIds`
- `nextId`
- `royalties`
- `paused`
