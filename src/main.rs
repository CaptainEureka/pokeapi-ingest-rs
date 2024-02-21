mod cli;
mod fetcher;
mod models;

use clap::Parser;
use fetcher::pokemon_fetcher::{FetchAllPokemon, PokeFetcher};
use fetcher::pokemon_fetcher::{FetchPokemon, NewFetch};

use cli::args::{Cli, Commands};
use reqwest::blocking::Client;

fn main() {
    let cli = Cli::parse();
    let client = reqwest::blocking::Client::new();

    match cli.command {
        Commands::Fetch { pokemon_id } => fetch_pokemon(client, &pokemon_id),
        Commands::Ingest {
            limit,
            offset,
            file_path,
        } => ingest_pokemon_data(client, limit, offset, &file_path),
    }
}

fn fetch_pokemon(client: Client, pokemon_id: &str) {
    let fetcher = PokeFetcher::new(client);
    match fetcher.fetch_pokemon_by_id(pokemon_id) {
        Ok(pokemon) => match serde_json::to_string_pretty(&pokemon) {
            Ok(json) => println!("{}", json),
            Err(err) => eprintln!("Error serializing Pokemon: {}", err),
        },
        Err(err) => eprintln!("Error fetching pokemon: {}", err),
    }
}

fn ingest_pokemon_data(client: Client, _limit: i32, _offset: i32, file_path: &str) {
    let fetcher = PokeFetcher::new(client);
    match fetcher.fetch_from_all() {
        Ok(data) => match data.write_json(file_path) {
            Ok(_) => println!("Data written to {}", file_path),
            Err(err) => eprintln!("Error writing to file: {}", err),
        },
        Err(err) => eprintln!("Error fetching PokemonData: {}", err),
    }
}
