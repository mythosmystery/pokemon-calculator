use serde::{Deserialize, Serialize};
use std::error::Error;

const DEX_PATH: &str = "assets/pokedex.json";
const MOVES_PATH: &str = "assets/moves.json";

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(DEX_PATH)?;
    let pokemon_data = serde_json::from_str::<Vec<PokemonData>>(&file)?;
    let file = std::fs::read_to_string(MOVES_PATH)?;
    let moves = serde_json::from_str::<Vec<Move>>(&file)?;
    println!("{:#?}", moves[0]);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    accuracy: Option<i32>,
    #[serde(rename = "ename")]
    name: String,
    power: Option<i32>,
    pp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pp: Option<i32>,
    #[serde(rename = "type")]
    kind: PokemonType,
    #[serde(skip_serializing_if = "Option::is_none")]
    tm: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    kind: PokemonData,
    nick_name: String,
    stats: Stats,
    moves: [Move; 4],
    ability: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct PokemonData {
    id: u16,
    name: Name,
    #[serde(rename = "type")]
    types: Vec<PokemonType>,
    base: Stats,
    species: String,
    description: String,
    evolution: Evolution,
    profile: Profile,
    image: Image,
}

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    sprite: String,
    thumbnail: String,
    hires: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    height: String,
    weight: String,
    egg: Option<Vec<String>>,
    ability: Vec<(String, String)>,
    gender: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Evolution {
    prev: Option<(String, String)>,
    next: Option<Vec<(String, String)>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    english: String,
    japanese: String,
    chinese: String,
    french: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stats {
    #[serde(rename = "HP")]
    hp: u32,
    #[serde(rename = "Attack")]
    attack: u32,
    #[serde(rename = "Defense")]
    defense: u32,
    #[serde(rename = "Sp. Attack")]
    special_attack: u32,
    #[serde(rename = "Sp. Defense")]
    special_defense: u32,
    #[serde(rename = "Speed")]
    speed: u32,
}
