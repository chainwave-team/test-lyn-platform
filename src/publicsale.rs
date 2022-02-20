// Handle the public sale process
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait PublicSale {
    // endpoints
    #[only_owner]
    #[endpoint(startPublicSale)]
    fn start_public_sale(&self) -> SCResult<()> {
        self._public_sale_status().set(true);
        Ok(())
    }
    /*
    Switch to true when the public sale starts
    */
    #[storage_mapper("publicSaleStatus")]
    fn _public_sale_status(&self) -> SingleValueMapper<bool>;
}
