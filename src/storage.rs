multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::zombie::Zombie;

#[multiversx_sc::module]
pub trait Storages {
    #[storage_mapper("dna_digits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("zombies_count")]
    fn zombies_count(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("zombies")]
    fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;

    #[storage_mapper("zombie_owner")]
    fn zombie_owner(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("owned_zombies")]
    fn owned_zombies(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;
}
