// Register the token to the blockchain

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait Register {
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

        self.token_name().set(&token_name);

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
                    can_change_owner: true,
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
        self._require_token_issued()?;

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

    // Require

    fn _require_token_issued(&self) -> SCResult<()> {
        require!(!self.token_id().is_empty(), "Token not issued");
        Ok(())
    }

    fn _require_local_roles_set(&self) -> SCResult<()> {
        let nft_token_id = self.token_id().get();
        let roles = self.blockchain().get_esdt_local_roles(&nft_token_id);

        require!(
            roles.has_role(&EsdtLocalRole::NftCreate),
            "NFTCreate role not set"
        );

        Ok(())
    }

    // Storage

    // eg: "LYNX-123456", defined on issue token
    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // eg: "Lynxee", defined on issue token
    #[view(getTokenName)]
    #[storage_mapper("tokenName")]
    fn token_name(&self) -> SingleValueMapper<ManagedBuffer>;
}
