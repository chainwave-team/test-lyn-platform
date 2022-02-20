elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const ROYALTIES_MAX: u32 = 10_000;

#[elrond_wasm::module]
pub trait Royalties {
    // endpoints

    #[only_owner]
    #[endpoint(setRoyalties)]
    fn set_royalties(&self, royalties: BigUint) -> SCResult<()> {
        require!(
            royalties <= BigUint::from(ROYALTIES_MAX),
            "Royalties cannot exceed 100%"
        );

        self._royalties().set(royalties);
        Ok(())
    }

    // require

    fn _require_royalties_set(&self) -> SCResult<()> {
        require!(!self._royalties().is_empty(), "Royalties not set");
        Ok(())
    }

    // storage

    #[storage_mapper("royalties")]
    fn _royalties(&self) -> SingleValueMapper<BigUint>;
}
