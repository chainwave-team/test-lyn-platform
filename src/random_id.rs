elrond_wasm::imports!();
elrond_wasm::derive_imports!();
const RESERVED_COUNT: usize = 10; // 10

#[elrond_wasm::module]
pub trait RandomId {
    // endpoints
    fn _fill_remaining_tokens(&self, supply: usize) -> SCResult<()> {
        let mut remaining_tokens_ids = self._remaining_tokens_ids().get();
        require!(
            remaining_tokens_ids.is_empty(),
            "remaining_tokens_ids already filled"
        );
        let start: u32 = RESERVED_COUNT as u32 + 1;
        remaining_tokens_ids = Vec::new();
        for i in start..=supply as u32 {
            remaining_tokens_ids.insert(0, i); // mb use push instead?
        }
        self._remaining_tokens_ids().set(remaining_tokens_ids);
        Ok(())
    }

    fn _shuffle_ids(&self) -> SCResult<()> {
        let mut remaining_tokens_ids = self._remaining_tokens_ids().get();
        require!(
            !remaining_tokens_ids.is_empty(),
            "remaining_tokens_ids is empty"
        );
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        for i in 0..((remaining_tokens_ids.len() - 1) / 2) {
            let rand_index = rand_source.next_usize_in_range(i, remaining_tokens_ids.len());
            let v1 = remaining_tokens_ids[rand_index];
            let v2 = remaining_tokens_ids[i];
            remaining_tokens_ids[i] = v1;
            remaining_tokens_ids[rand_index] = v2;
        }
        self._remaining_tokens_ids().set(remaining_tokens_ids);
        Ok(())
    }
    fn _generate_random_id(&self) -> SCResult<u32> {
        let mut remaining_tokens_ids = self._remaining_tokens_ids().get();
        require!(
            remaining_tokens_ids.len() > 0,
            "All public token have been generated"
        );
        let uid = remaining_tokens_ids[remaining_tokens_ids.len() - 1];
        remaining_tokens_ids.pop();
        self._remaining_tokens_ids().set(remaining_tokens_ids);
        Ok(uid)
    }
    #[storage_mapper("remainingTokens")]
    fn _remaining_tokens_ids(&self) -> SingleValueMapper<Vec<u32>>;

    #[view(getRemainingCount)]
    fn get_remaining_count(&self) -> SCResult<u32> {
        let remaining_tokens_ids = self._remaining_tokens_ids().get();
        Ok(remaining_tokens_ids.len() as u32)
    }
}
