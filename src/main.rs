fn main() {}

enum PokemonName {
    Pikachu,
}

enum PokemonType {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

struct Pokemon {
    number: u32,
    name: PokemonName,
    types: Vec<PokemonType>,
    stats: Stats,
}

struct Stats {
    total: i32,
    hp: i32,
    attack: i32,
    defense: i32,
    special_attack: i32,
    special_defense: i32,
    speed: i32,
}
