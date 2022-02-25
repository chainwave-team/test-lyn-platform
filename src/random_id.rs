elrond_wasm::imports!();
elrond_wasm::derive_imports!();
const RESERVED_COUNT: usize = 10; // 10

#[elrond_wasm::module]
pub trait RandomId {
    // endpoints
    fn _fill_remaining_tokens(&self, supply: usize) -> SCResult<()> {
        require!(
            self.token_ids_shuffled().is_empty(),
            "remaining_tokens_ids already filled"
        );
        let start: usize = RESERVED_COUNT + 1;
        let mut tokens_vec = Vec::new();
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        for i in start..=supply {
            let vec_len = tokens_vec.len();
            tokens_vec.push(i as u32);
            if (vec_len > 2) {
                let rand_index = rand_source.next_usize_in_range(0, vec_len);
                tokens_vec.swap(rand_index, 0);
            }
        }
        self.token_ids_shuffled()
            .extend_from_slice(&tokens_vec.as_slice());
        Ok(())
    }

    fn _generate_random_id(&self, _sold: usize) -> SCResult<u32> {
        // get index starts at 1 
        Ok(self.token_ids_shuffled().get(_sold + 1))
    }

    #[storage_mapper("remainingTokens")]
    fn token_ids_shuffled(&self) -> VecMapper<u32>;
}
