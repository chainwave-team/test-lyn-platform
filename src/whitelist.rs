elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const MAX_WHITELISTED_USERS: usize = 10; // 200

// When a whitelisted user uses its advantage,
// then he is remove from the whitelist.

#[elrond_wasm::module]
pub trait Whitelist {
    // endpoints

    #[only_owner]
    #[endpoint(whiteList)]
    fn add_to_whitelist(&self, address: ManagedAddress) -> SCResult<()> {
        let total: usize = self._whitelist_total().get();
        require!(total <= MAX_WHITELISTED_USERS, "The whitelist is full");

        // add to the whitelist
        self._whitelist().insert(address);

        // increment total count
        if self._whitelist_total().is_empty() {
            self._whitelist_total().set(1);
        } else {
            self._whitelist_total().update(|&mut prev| prev + 1);
        }

        Ok(())
    }

    fn _is_whitelisted(&self, address: &ManagedAddress) -> bool {
        self._whitelist().contains(address)
    }

    fn _remove_from_whitelist(&self, address: &ManagedAddress) -> bool {
        self._whitelist().remove(address)
    }

    // storage
    // current whitelisted addresses
    #[storage_mapper("whitelistedAddresses")]
    fn _whitelist(&self) -> SetMapper<ManagedAddress>;

    // all time counter
    #[storage_mapper("whitelistTotal")]
    fn _whitelist_total(&self) -> SingleValueMapper<usize>;
}
