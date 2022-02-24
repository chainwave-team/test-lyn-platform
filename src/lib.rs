#![no_std]

// All the nft creation methods boilerplate code is in the CreateNFT mod
// Here there is some stuff like
// - the public endpoints
// - the whitelist logic
// - the dynamic price logic
// - the giveaway logic

// Be careful, there is some config available in the CreateNFT mod.

extern crate alloc;

// use alloc::{string::String, vec::Vec};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod publicsale;
mod random_id;
mod register;
mod royalties;
mod utils;
mod whitelist;

use crate::utils::{build_attributes, build_name, build_uris, str_to_buffer};

// TODO: WARNING - FAKE DATA TO TEST
const ON_SALE_SUPPLY: usize = 2800; // 2800;
const MAX_SUPPLY: usize = 3000; // 3000;
const MAX_MINT_COUNT_BY_ADDRESS: usize = 5;
const ONE_EGLD: u64 = 1_000_000_000_000_000_000;

// TODO: Pass it as function args (in init ideally)
const IMAGE_CID: &str = "bafybeiecb3p6mydrb3br4a3fipxzca7tcwvp4ilqlqmjuri4hkfzpuqe3u";
const JSON_CID: &str = "bafybeibfcxtinf2vnwaq6a5brqqftrhb74w4fcuc7uutwf36h5v5banyma";

#[elrond_wasm::contract]
pub trait NftMinter:
    register::Register
    + royalties::Royalties
    + whitelist::Whitelist
    + random_id::RandomId
    + publicsale::PublicSale
{
    // constructor called on deploy
    #[init]
    fn init(
        &self,
        royalties: BigUint, // eg: 1000 (10%)
        tags: ManagedBuffer, // eg: animal,art
                            // json_cid: ManagedBuffer,
                            // image_cid: ManagedBuffer,
    ) -> SCResult<()> {
        self.set_royalties(royalties)?;
        self._tags().set(&tags);
        self._json_cid().set(&str_to_buffer(JSON_CID));
        self._image_cid().set(&str_to_buffer(IMAGE_CID));
        self._fill_remaining_tokens(MAX_SUPPLY)?;
        self._shuffle_ids()?;
        self._public_sale_status().set(false);
        Ok(())
    }

    // endpoints

    /// This function is the private version of mint, but here you have more control.
    #[only_owner]
    #[endpoint(giveaway)]
    fn giveaway(
        &self,
        receiver: &ManagedAddress,
        #[var_args] id: OptionalArg<u32>,
    ) -> SCResult<()> {
        let next_id: u32 = match id {
            OptionalArg::Some(index) => index,
            OptionalArg::None => self._generate_random_id()?,
        };

        // Mint
        let nft_nonce = self._mint(next_id)?;

        // Send the fresh minted NFT to the given "receiver" address
        self.send().direct(
            &receiver,                // to
            &self.token_id().get(),   // token_identifier
            nft_nonce as u64,         // nonce
            &BigUint::from(1 as u32), // amount (must be 1 for NFT)
            &[],                      // data (empty)
        );
        Ok(())
    }
    /// Run multiple time "giveaway" methods to send many tokens
    #[only_owner]
    #[endpoint(giveawayMany)]
    fn giveaway_many(&self, receiver: &ManagedAddress, count: u32) -> SCResult<()> {
        for _ in 0..count {
            self.giveaway(receiver, OptionalArg::None)?;
        }
        Ok(())
    }

    // endpoints - public

    /// Public method to mint a random NFT, this is a payable function.
    #[payable("EGLD")]
    #[endpoint(mint)]
    fn mint(&self, #[payment_amount] payment_amount: BigUint) -> SCResult<()> {
        // if there are still tokens to sell
        let sold_minted_count = self.get_sold_count();
        require!(
            sold_minted_count < ON_SALE_SUPPLY,
            "All on sale token have been minted"
        );

        // caller still could mint
        let is_public_sale = self._public_sale_status().get();
        let caller = self.blockchain().get_caller();
        let caller_mint_count = self._sold_count_by_address(&caller).get();
        let is_whitelisted = self._is_whitelisted(&caller);

        if (!is_public_sale) {
            require!(is_whitelisted, "Caller is not whitelisted");
        }
        require!(
            caller_mint_count < MAX_MINT_COUNT_BY_ADDRESS,
            "Max mint per person reached"
        );

        // Should be able to pay
        let price = self.get_mint_price(&caller);
        require!(payment_amount == price, "Invalid amount as payment");

        // Mint
        let id = self._generate_random_id()?;
        let nft_nonce = self._mint(id)?;

        // Pay the mint cost
        self.send().direct(
            &self.blockchain().get_owner_address(), // to
            &TokenIdentifier::egld(),               // token
            0,                                      // nonce
            &payment_amount,                        // amount
            &[],                                    // data
        );

        // Send the fresh minted NFT to the caller
        self.send().direct(
            &caller,                  // to
            &self.token_id().get(),   // token
            nft_nonce as u64,         // nonce
            &BigUint::from(1 as u32), // amount
            &[],                      // data
        );

        // increment mint by caller
        self._sold_count_by_address(&caller)
            .set(caller_mint_count + 1);

        // increment sold mint count
        self._sold_minted_ids().insert(id);

        Ok(())
    }

    // views

    /// This function return the current mint price based on how many have been mined.
    /// The get_mint_price works on the 2700 on sale nfts, don't use it for giveaway, it makes sense.

    #[view(getMintPrice)]
    fn get_mint_price(&self, caller: &ManagedAddress) -> BigUint {
        const CENT: u64 = ONE_EGLD / 100;
        let is_whitelisted = self._is_whitelisted(&caller);

        /*
        if user whitelisted and the public sale has not started yet,
        return 0.1 EGLD
        */
        if is_whitelisted {
            self._remove_from_whitelist(&caller);
            return BigUint::from(10 * CENT); // 0.1 EGLD
        }

        let already_sold = self._sold_minted_ids().len() as usize;
        let mint_price = match already_sold {
            0..=800 => 20 * CENT,  // 1
            0..=1300 => 25 * CENT, // 2
            0..=1800 => 30 * CENT, // 3
            0..=2300 => 35 * CENT, //4
            _ => 40 * CENT,        //5
        };
        BigUint::from(mint_price)
    }

    #[view(getMintedCount)]
    fn get_minted_count(&self) -> usize {
        self._minted_ids().len()
    }

    #[view(getSoldCount)]
    fn get_sold_count(&self) -> usize {
        self._sold_minted_ids().len()
    }

    // private

    /// Wrap the NFT creation with logic and checking
    fn _mint(&self, id: u32) -> SCResult<u32> {
        self._require_token_issued()?;
        self._require_local_roles_set()?;
        self._require_royalties_set()?;
        require!(
            self._minted_ids().len() < MAX_SUPPLY,
            "All token have been minted"
        );
        require!(!self._minted_ids().contains(&id), "Token already minted");
        require!(id > 0, "Token id must be greater than 0");
        require!((id as usize) <= MAX_SUPPLY, "Token id out of collection");

        // Mint
        let nonce = self._create_nft(id)?;

        // Increment total mint count
        self._minted_ids().insert(id);

        Ok(nonce as u32)
    }

    /// ATTENTION: This function only create the NFT without any check
    fn _create_nft(&self, id: u32) -> SCResult<u32> {
        // Build token metadata
        let token_id = self.token_id().get();
        let token_name = self.token_name().get();
        let image_cid = self._image_cid().get();
        let json_cid = self._json_cid().get();
        let tags = self._tags().get();
        let royalties = self._royalties().get();
        let name = build_name(&token_name, &id);
        let uris = build_uris(&image_cid, &id);
        let attributes = build_attributes(&json_cid, &tags, &id);
        let attributes_hash = self
            .crypto()
            .sha256_legacy(&attributes.to_boxed_bytes().as_slice());
        let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());
        let ntf_amount = BigUint::from(1 as u32);

        // send tx
        let nonce = self.send().esdt_nft_create(
            &token_id,
            &ntf_amount,
            &name,
            &royalties,
            &hash_buffer,
            &attributes,
            &uris,
        );

        Ok(nonce as u32)
    }

    // storage

    #[storage_mapper("soldMintedIds")]
    fn _sold_minted_ids(&self) -> SetMapper<u32>;

    #[storage_mapper("mintCountByAddress")]
    fn _sold_count_by_address(&self, address: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("jsonCid")]
    fn _json_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("imageCid")]
    fn _image_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("tags")]
    fn _tags(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("mintedIds")]
    fn _minted_ids(&self) -> SetMapper<u32>;
}
