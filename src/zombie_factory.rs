multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie::Zombie};

#[multiversx_sc::module]
pub trait ZombieFactory: storage::Storages {
    fn create_zombie(&self, caller: ManagedAddress, name: ManagedBuffer, dna: u64) {
        self.zombies_count().update(|id| {
            self.new_zombie_event(*id, &name, dna);
            self.zombies(id).set(Zombie { name, dna });
            self.owned_zombies(&caller).insert(*id);
            self.zombie_owner(id).set(caller);
            *id += 1;
        });
    }

    #[view]
    fn generate_random_dna(&self) -> u64 {
        // start here
        let mut rand_source = RandomnessSource::new();
        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        rand_source.next_u64_in_range(0u64, max_dna_value)
    }

    #[endpoint]
    fn create_random_zombie(&self, name: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        let rand_dna = self.generate_random_dna();
        self.create_zombie(caller, name, rand_dna);
    }

    #[event("new_zombie_event")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        name: &ManagedBuffer,
        #[indexed] dna: u64,
    );
}
