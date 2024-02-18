use std::{error::Error, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonId(i32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    id: PokemonId,
    name: String,
    abilities: Vec<PokemonAbility>,
    types: Vec<PokemonType>,
    height: f32,
    weight: f32,
    base_experience: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbility {
    is_hidden: bool,
    slot: i32,
    ability: Ability,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonType {
    slot: i32,
    #[serde(rename = "type")]
    poke_type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonListItem {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokeApiResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<PokemonListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct PokemonData {
    pub results: Vec<Pokemon>,
}

impl PokemonData {
    pub fn new(results: Vec<Pokemon>) -> Self {
        Self { results }
    }

    pub fn write_json(&self, fp: &str) -> Result<(), Box<dyn Error>> {
        // Create file to write to
        let file = File::create(fp)?;

        // Write using serde
        serde_json::to_writer(&file, &self)?;

        Ok(())
    }
}
