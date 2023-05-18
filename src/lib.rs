#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
mod zombie;
mod zombie_factory;
mod zombie_feeding;

#[multiversx_sc::contract]
pub trait ZombiesContract:
    zombie_factory::ZombieFactory + zombie_feeding::ZombieFeeding + storage::Storages
{
    #[init]
    fn init(&self) {
        self.dna_digits().set(16u8);
        self.zombies_count().set(1usize);
    }
}
