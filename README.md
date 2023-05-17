# CrytoZombies course 

### https://cryptozombies.io/en/multiversx  

### Smart Contract Diagram:

```mermaid
classDiagram
    class ZombieContract{
        +init 
    }
    class SingleValueMapper
    SingleValueMapper <|-- dna_digits
    class dna_digits {
        -> u8
    }
    SingleValueMapper <|-- zombies_count
    class zombies_count {
        -> usize
    }
    SingleValueMapper <|-- zombie
    class zombie {
        id ->        Zombie
    }
    class Zombie {
        name: ManagedBuffer
        dna: u64
    }
    create_zombie --|> Zombie  
    class create_zombie {
        zombie(id) -> new Zombie
        zombies_count +1
    }
    zombie <|-- create_zombie  
    zombies_count  <|-- create_zombie 
    class generate_random_dna{
        -> u64
    }
    class create_random_zombie {
        create_zombie(name, dna)
     }
    create_random_zombie --|> generate_random_dna 
    create_zombie <|-- create_random_zombie
    generate_random_dna --|> dna_digits
```

 