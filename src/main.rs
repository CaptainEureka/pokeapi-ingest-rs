mod cli;
mod fetcher;
mod models;

use clap::Parser;
use fetcher::pokemon_fetcher::{FetchPokemon, PokeFetcher};

use cli::args::{Cli, Commands};
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Fetch { pokemon_id } => fetch_pokemon(client, &pokemon_id).await,
        // TODO: Implement the Ingest command as Enum?
        Commands::Ingest {
            all,
            limit: _,
            offset: _,
            file_path,
        } if all => ingest_all_pokemon_data(client, &file_path).await,
        Commands::Ingest {
            all: _,
            limit,
            offset,
            file_path,
        } => ingest_pokemon_data(client, limit, offset, &file_path).await,
    }
}

async fn fetch_pokemon(client: Client, pokemon_id: &str) {
    let fetcher = PokeFetcher::new(client);
    match fetcher.fetch_pokemon_by_id(pokemon_id).await {
        Ok(pokemon) => match serde_json::to_string_pretty(&pokemon) {
            Ok(json) => println!("{}", json),
            Err(err) => eprintln!("Error serializing Pokemon: {}", err),
        },
        Err(err) => eprintln!("Error fetching pokemon: {}", err),
    }
}

async fn ingest_pokemon_data(client: Client, limit: i32, offset: i32, file_path: &str) {
    let fetcher = PokeFetcher::new(client);

    let buffer_size: usize = std::env::var("BUFFER_SIZE")
        .map_or(50, |buffer_size_str| buffer_size_str.parse().unwrap_or(50));

    match fetcher
        .fetch_with_limit_and_offset(&limit, &offset, buffer_size)
        .await
    {
        Ok(data) => match data.write_json(file_path) {
            Ok(_) => println!("Data written to {}", file_path),
            Err(err) => eprintln!("Error writing to file: {}", err),
        },
        Err(err) => eprintln!("Error fetching PokemonData: {}", err),
    }
}

async fn ingest_all_pokemon_data(client: Client, file_path: &str) {
    let fetcher = PokeFetcher::new(client);
    match fetcher.fetch_all().await {
        Ok(data) => match data.write_json(file_path) {
            Ok(_) => println!("Data written to {}", file_path),
            Err(err) => eprintln!("Error writing to file: {}", err),
        },
        Err(err) => eprintln!("Error fetching PokemonData: {}", err),
    }
}
