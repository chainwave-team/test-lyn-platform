#![no_std]

extern crate alloc;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000;
const MAX_SUPPLY: usize = 3000;

const PRICE_STAGE_1: u64 = 200_000_000_000_000_000; // 0.2 EGLD
const PRICE_STAGE_2: u64 = 400_000_000_000_000_000; // 0.4 EGLD
const PRICE_STAGE_3: u64 = 600_000_000_000_000_000; // 0.6 EGLD

const IMAGE_EXT: &str = ".png";
const IPFS_SCHEME: &str = "ipfs://";
const METADATA_KEY_NAME: &str = "metadata:";
const METADATA_FILE_EXTENSION: &str = ".json";
const ATTR_SEPARATOR: &str = ";";
const URI_SLASH: &str = "/";
const TAGS_KEY_NAME: &str = "tags:";

// #[derive(TypeAbi, TopEncode, TopDecode)]
// pub struct ExampleAttributes {
//     pub creation_timestamp: u64,
// }

// IDEA: use a struct for the Attributes?
// IDEA: change owner?
// IDEA: Free mint for the team?
// IDEA: impl erc-721?
// TODO: Test each endpoint

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

    #[payable("EGLD")]
    #[endpoint(mint)]
    fn mint(&self, #[payment_amount] payment_amount: BigUint) -> SCResult<()> {
        // Check if we can perform the mint
        self.require_token_issued()?;
        self.require_local_roles_set()?;
        self.require_is_not_paused()?;
        let mint_price = self.get_mint_price();
        let total_minted = self.minted_ids().len();
        require!(total_minted < MAX_SUPPLY, "All token have been minted");
        require!(payment_amount == mint_price, "Invalid amount as payment");

        // Get Token metadata
        let token_identifier = self.token_id().get();
        let id = self.generate_next_id();
        let name = self.build_name(&id);
        let uris = self.build_uris(&id);
        let royalties = self.royalties().get();
        let attributes = self.build_attributes(&id);
        let attributes_hash = self
            .crypto()
            .sha256_legacy(&attributes.to_boxed_bytes().as_slice());
        let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());

        // Prepare payment - use EGLD to pay
        let payment_token = TokenIdentifier::egld();
        let payment_nonce: u64 = 0;
        let ntf_amount = BigUint::from(NFT_AMOUNT);
        let caller = self.blockchain().get_caller();
        let owner = self.blockchain().get_owner_address();

        // Pay the mint cost
        self.send()
            .direct(&owner, &payment_token, payment_nonce, &payment_amount, &[]);

        // Create the NFT
        let nft_nonce = self.send().esdt_nft_create(
            &token_identifier,
            &ntf_amount,
            &name,
            &royalties,
            &hash_buffer,
            &attributes,
            &uris,
        );

        self.minted_ids().insert(id);

        // Send the fresh minted NFT to the caller
        self.send()
            .direct(&caller, &token_identifier, nft_nonce, &ntf_amount, &[]);

        Ok(())
    }

    // views

    #[view(getMintPrice)]
    fn get_mint_price(&self) -> BigUint {
        let mint_price = match self.minted_ids().len() {
            supply if supply < 1000 => PRICE_STAGE_1,
            supply if supply < 2000 => PRICE_STAGE_2,
            _ => PRICE_STAGE_3,
        };

        BigUint::from(mint_price)
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

        // metadata: cid
        attributes.append(&self.str_to_buffer(METADATA_KEY_NAME));
        attributes.append(&self.json_cid().get());
        attributes.append(&self.str_to_buffer(URI_SLASH));
        attributes.append(&self.u32_to_buffer(index));
        attributes.append(&self.str_to_buffer(METADATA_FILE_EXTENSION));
        attributes.append(&self.str_to_buffer(ATTR_SEPARATOR));
        // tags
        attributes.append(&self.str_to_buffer(TAGS_KEY_NAME));
        attributes.append(&self.tags().get());

        attributes
    }

    // Note: The 3000 NFTs have already been uploaded on IPFS, even they haven't been minted.
    // So, to mint them randomly, we select the next id randomly.
    fn generate_next_id(&self) -> u32 {
        // get random number
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        // TODO: May be optimized by looking for the resting range instead whole range
        let mut rand_index = rand_source.next_u32_in_range(1, 3000);

        while self.minted_ids().contains(&rand_index) {
            rand_index = rand_source.next_u32_in_range(1, 3000);
        }

        rand_index
    }

    fn str_to_buffer(&self, string: &str) -> ManagedBuffer {
        ManagedBuffer::new_from_bytes(string.as_bytes())
    }

    fn bytes_to_buffer(&self, string: &str) -> ManagedBuffer {
        ManagedBuffer::new_from_bytes(string.as_bytes())
    }

    fn u32_to_buffer(&self, string: &u32) -> ManagedBuffer {
        use alloc::string::ToString;
        ManagedBuffer::new_from_bytes(string.to_string().as_bytes())
    }

    fn require_token_issued(&self) -> SCResult<()> {
        require!(!self.token_id().is_empty(), "Token not issued");
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

    #[storage_mapper("mintedIds")]
    fn minted_ids(&self) -> SetMapper<u32>;

    #[storage_mapper("nextId")]
    fn next_id(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("royalties")]
    fn royalties(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;
}
