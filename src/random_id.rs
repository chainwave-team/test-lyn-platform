elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const RESERVED_COUNT: usize = 8; // team

#[elrond_wasm::module]
pub trait RandomId {
    // endpoints

    fn _generate_random_id(&self) -> SCResult<u32> {
        let remaining_tokens = self._remaining_tokens_ids();
        require!(remaining_tokens.len() > 0, "No more tokens available");

        let start_index = RESERVED_COUNT + 1;
        let end_index = remaining_tokens.len() + 1; // (min, max] range needs tests

        require!(
            start_index < end_index,
            "All public token have been generated"
        );

        let mut rand = RandomnessSource::<Self::Api>::new();
        let rand_index = rand.next_usize_in_range(start_index, end_index);

        for (i, uid) in remaining_tokens.iter().enumerate() {
            if i == rand_index {
                return Ok(uid);
            }
        }

        sc_error!("Error while generating random id")

        // Ok(remaining_tokens
        //     // .iter()
        //     .enumerate()
        //     .filter(|(i, _)| i == &rand_index))
    }

    /// Set the initial remaining data, will be decremented at each mint
    fn _fill_remaining_tokens(&self, supply: usize) -> SCResult<()> {
        let mut remaining_tokens_ids = self._remaining_tokens_ids();

        require!(
            remaining_tokens_ids.is_empty(),
            "remaining_tokens_ids already filled"
        );
        for i in 1..=supply as u32 {
            remaining_tokens_ids.insert(i);
        }

        Ok(())
    }

    fn _remove_id_from_remaining_list(&self, uid: u32) -> bool {
        let mut remaining = self._remaining_tokens_ids();
        remaining.remove(&uid)
    }

    // view
    #[view(getRemainingCount)]
    fn get_remaining_count(&self) -> usize {
        self._remaining_tokens_ids().len()
    }
    // storage
    #[storage_mapper("remainingTokens")]
    fn _remaining_tokens_ids(&self) -> SetMapper<u32>;
}
