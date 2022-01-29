#![no_std]

extern crate alloc;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000;
const MAX_SUPPLY: u32 = 3000;
const ON_SALE_SUPPLY: u32 = 2700;
const ONE_EGLD: u64 = 1_000_000_000_000_000_000;
const IMAGE_EXT: &str = ".png";
const IPFS_SCHEME: &str = "ipfs://";
const METADATA_KEY_NAME: &str = "metadata:";
const METADATA_FILE_EXTENSION: &str = ".json";
const ATTR_SEPARATOR: &str = ";";
const URI_SLASH: &str = "/";
const TAGS_KEY_NAME: &str = "tags:";

#[elrond_wasm::contract]
pub trait NftMinter {
    // constructor called on deploy
    #[init]
    fn init(
        &self,
        royalties: BigUint, // eg: 1000 (10%)
        json_cid: ManagedBuffer,
        image_cid: ManagedBuffer,
        tags: ManagedBuffer, // eg: animal,art
    ) -> SCResult<()> {
        self.set_royalties(royalties)?;
        self.json_cid().set(&json_cid);
        self.image_cid().set(&image_cid);
        self.tags().set(&tags);

        Ok(())
    }

    // endpoints - owner-only

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        #[payment] issue_cost: BigUint,
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
    ) -> SCResult<AsyncCall> {
        require!(self.token_id().is_empty(), "Token already issued");

        Ok(self
            .send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost,
                &token_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback()))
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) -> SCResult<AsyncCall> {
        self.require_token_issued()?;

        Ok(self
            .send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.token_id().get(),
                (&[EsdtLocalRole::NftCreate][..]).into_iter().cloned(),
            )
            .async_call())
    }

    /// Run multiple time "giveaway" methods to send many tokens
    #[only_owner]
    #[endpoint(giveawayMany)]
    fn giveaway_many(&self, receiver: ManagedAddress, count: u32) -> SCResult<()> {
        for _ in 0..count {
            let next_id = self.next_id().get();
            self.generate_random_next_id()?;
            self.giveaway(&receiver, OptionalArg::Some(next_id))?;
        }
        Ok(())
    }

    /// This function is the private version of mint, but here you have more control.
    #[only_owner]
    #[endpoint(giveaway)]
    fn giveaway(
        &self,
        receiver: &ManagedAddress,
        #[var_args] id: OptionalArg<u32>,
    ) -> SCResult<()> {
        let id: u32 = match id {
            OptionalArg::Some(index) => {
                self.require_token_id_available(&index)?;
                index
            }
            OptionalArg::None => {
                let next_id = self.next_id().get();
                self.generate_random_next_id()?;
                next_id
            }
        };

        // Mint
        let nft_nonce = self.create_nft(id)?;

        // Send the fresh minted NFT to the given "receiver" address
        self.send().direct(
            &receiver,                  // to
            &self.token_id().get(),     // token_identifier
            nft_nonce,                  // nonce
            &BigUint::from(NFT_AMOUNT), // amount (must be 1 for NFT)
            &[],                        // data (empty)
        );

        Ok(())
    }

    #[only_owner]
    #[endpoint(setRoyalties)]
    fn set_royalties(&self, royalties: BigUint) -> SCResult<()> {
        require!(
            royalties <= BigUint::from(ROYALTIES_MAX),
            "Royalties cannot exceed 100%"
        );
        self.royalties().set(royalties);
        Ok(())
    }

    #[only_owner]
    #[endpoint(pauseMinting)]
    fn pause_minting(&self) -> SCResult<()> {
        let paused = true;
        self.paused().set(&paused);

        Ok(())
    }

    #[only_owner]
    #[endpoint(startMinting)]
    fn start_minting(&self) -> SCResult<()> {
        self.paused().clear();

        Ok(())
    }

    // endpoints - public

    /// Public method to mint a random NFT, this is a payable function.
    #[payable("EGLD")]
    #[endpoint(mint)]
    fn mint(&self, #[payment_amount] payment_amount: BigUint) -> SCResult<()> {
        self.require_tokens_to_be_sold()?;
        self.require_is_not_paused()?;
        self.require_there_is_enough_to_pay(&payment_amount)?;

        // Mint
        let nft_nonce = self.create_nft(self.next_id().get())?;
        self.generate_random_next_id()?;

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
            &self.blockchain().get_caller(), // to
            &self.token_id().get(),          // token
            nft_nonce,                       // nonce
            &BigUint::from(NFT_AMOUNT),      // amount
            &[],                             // data
        );

        Ok(())
    }

    // views

    /// This function return the current mint price based on how many have been mined.
    /// The get_mint_price works on the 2700 on sale nfts, don't use it for giveaway, it makes sense.
    #[view(getMintPrice)]
    fn get_mint_price(&self) -> BigUint {
        const CENT: u64 = ONE_EGLD / 100;

        let mint_price = match self.sold_minted_ids().len() {
            // range from 1 to 2700
            supply if supply < 200 => 10 * CENT, // the next 200 are at 0,1 egld
            supply if supply < 700 => 20 * CENT, // the next 500 are at 0,2 egld
            supply if supply < 1200 => 25 * CENT, // the next 500 are at 0,25 egld
            supply if supply < 1700 => 30 * CENT, // the next 500 are at 0,3 egld
            supply if supply < 2200 => 35 * CENT, // the next 500 are at 0,35 egld
            _ => 40 * CENT,                      // the last 500 are at 0,4 egld
        };

        BigUint::from(mint_price)
    }

    // Convert to hex

    #[view(getMintedCount)]
    fn get_minted_count(&self) -> usize {
        self.minted_ids().len()
    }

    #[view(getSoldCount)]
    fn get_sold_count(&self) -> usize {
        self.sold_minted_ids().len()
    }

    // callbacks

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.token_id().set(&token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let (returned_tokens, token_id) = self.call_value().payment_token_pair();
                if token_id.is_egld() && returned_tokens > 0 {
                    self.send()
                        .direct(&caller, &token_id, 0, &returned_tokens, &[]);
                }
            }
        }
    }

    // private

    fn create_nft(&self, id: u32) -> SCResult<u64> {
        self.require_token_issued()?;
        self.require_local_roles_set()?;
        self.require_tokens_to_be_mined()?;

        // Build token metadata
        let token_identifier = self.token_id().get();
        let name = self.build_name(&id);
        let uris = self.build_uris(&id);
        let royalties = self.royalties().get();
        let attributes = self.build_attributes(&id);
        let attributes_hash = self
            .crypto()
            .sha256_legacy(&attributes.to_boxed_bytes().as_slice());
        let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());
        let ntf_amount = BigUint::from(NFT_AMOUNT);

        // Mint
        let nonce = self.send().esdt_nft_create(
            &token_identifier,
            &ntf_amount,
            &name,
            &royalties,
            &hash_buffer,
            &attributes,
            &uris,
        );

        self.minted_ids().insert(id);

        Ok(nonce)
    }

    /// Return the NFT item name like "Lynxee #420"
    fn build_name(&self, id: &u32) -> ManagedBuffer {
        let mut name = self.token_name().get();
        name.append(&self.str_to_buffer(" #"));
        name.append(&self.u32_to_buffer(id));
        name
    }

    /// Build a vector with the image uri inside
    fn build_uris(&self, index: &u32) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();

        let mut img_ipfs_uri = self.str_to_buffer(IPFS_SCHEME);
        img_ipfs_uri.append(&self.image_cid().get());
        img_ipfs_uri.append(&self.str_to_buffer(URI_SLASH));
        img_ipfs_uri.append(&self.u32_to_buffer(index));
        img_ipfs_uri.append(&self.str_to_buffer(IMAGE_EXT));

        uris.push(img_ipfs_uri);
        uris
    }

    /// Build the attributes Buffer including the metadata json
    /// Format: tags:tag1,tag2;metadata:ipfsCID/1.json
    fn build_attributes(&self, index: &u32) -> ManagedBuffer {
        let mut attributes = ManagedBuffer::new();

        // metadata:cid;
        attributes.append(&self.str_to_buffer(METADATA_KEY_NAME));
        attributes.append(&self.json_cid().get());
        attributes.append(&self.str_to_buffer(URI_SLASH));
        attributes.append(&self.u32_to_buffer(index));
        attributes.append(&self.str_to_buffer(METADATA_FILE_EXTENSION));
        attributes.append(&self.str_to_buffer(ATTR_SEPARATOR));
        // tags:tag1,tag2
        attributes.append(&self.str_to_buffer(TAGS_KEY_NAME));
        attributes.append(&self.tags().get());

        attributes
    }

    /// The 3000 NFTs have already been uploaded on IPFS, even they haven't been minted.
    /// So, excepted for special ones, we'll mint them randomly to mint them randomly.
    /// This function generate randomly the next available id.
    // TODO: May be optimized by looking for the resting range instead whole range
    fn generate_random_next_id(&self) -> SCResult<()> {
        // It starts at 11 because the ten firsts are reserved.
        const STARTING_INDEX: u32 = 11;

        // get random number
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        let mut rand_index = rand_source.next_u32_in_range(STARTING_INDEX, MAX_SUPPLY);

        while self.minted_ids().contains(&rand_index) {
            rand_index = rand_source.next_u32_in_range(STARTING_INDEX, MAX_SUPPLY);
        }

        self.next_id().set(rand_index);
        Ok(())
    }

    fn str_to_buffer(&self, string: &str) -> ManagedBuffer {
        ManagedBuffer::new_from_bytes(string.as_bytes())
    }

    fn u32_to_buffer(&self, string: &u32) -> ManagedBuffer {
        use alloc::string::ToString;
        ManagedBuffer::new_from_bytes(string.to_string().as_bytes())
    }

    // Require functions

    fn require_token_issued(&self) -> SCResult<()> {
        require!(!self.token_id().is_empty(), "Token not issued");
        Ok(())
    }

    fn require_there_is_enough_to_pay(&self, payment_amount: &BigUint) -> SCResult<()> {
        require!(
            payment_amount == &self.get_mint_price(),
            "Invalid amount as payment"
        );
        Ok(())
    }

    fn require_token_id_available(&self, id: &u32) -> SCResult<()> {
        require!(!self.minted_ids().contains(id), "Token id is already taken");
        Ok(())
    }

    fn require_tokens_to_be_mined(&self) -> SCResult<()> {
        require!(
            (self.minted_ids().len() as u32) < MAX_SUPPLY,
            "All token have been minted"
        );
        Ok(())
    }

    fn require_tokens_to_be_sold(&self) -> SCResult<()> {
        require!(
            (self.sold_minted_ids().len() as u32) < ON_SALE_SUPPLY,
            "All on sale token have been minted"
        );
        Ok(())
    }

    fn require_is_not_paused(&self) -> SCResult<()> {
        require!(
            self.paused().is_empty(),
            "The minting is paused or haven't started yet!"
        );
        Ok(())
    }

    fn require_local_roles_set(&self) -> SCResult<()> {
        let nft_token_id = self.token_id().get();
        let roles = self.blockchain().get_esdt_local_roles(&nft_token_id);

        require!(
            roles.has_role(&EsdtLocalRole::NftCreate),
            "NFTCreate role not set"
        );

        Ok(())
    }

    // storage

    // eg: "LYNX-123456", defined on issue token
    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // eg: "Lynxee", defined on issue token
    #[view(getTokenName)]
    #[storage_mapper("tokenName")]
    fn token_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getJsonCid)]
    #[storage_mapper("jsonCid")]
    fn json_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getImageCid)]
    #[storage_mapper("imageCid")]
    fn image_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("tags")]
    fn tags(&self) -> SingleValueMapper<ManagedBuffer>;

    // Set map to store all minted nfts
    #[storage_mapper("mintedIds")]
    fn minted_ids(&self) -> SetMapper<u32>;

    // Set map to store sold minted nfts
    #[storage_mapper("soldMintedIds")]
    fn sold_minted_ids(&self) -> SetMapper<u32>;

    #[storage_mapper("royalties")]
    fn royalties(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;

    // We save the next id in a var to be able to mock it in unit tests
    #[storage_mapper("nextId")]
    fn next_id(&self) -> SingleValueMapper<u32>;
}
