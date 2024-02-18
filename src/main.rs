mod cli;
mod fetcher;
mod models;

use cli::args::get_args;
use fetcher::pokemon_fetcher::{fetch_pokemon_by_id, PokeFetcher};

fn main() {
    let matches = get_args().get_matches();
    let client = reqwest::blocking::Client::new();

    match matches.subcommand() {
        Some(("fetch", sub_matches)) => {
            let pokemon_id = sub_matches
                .get_one::<String>("pokemon_id")
                .expect("required");
            let pokemon = fetch_pokemon_by_id(client, pokemon_id).unwrap();

            println!("{}", serde_json::to_string_pretty(&pokemon).unwrap())
        }
        Some(("ingest", sub_matches)) => {
            let poke_fetch = PokeFetcher::new(client);
            let offset: Option<&i32> = sub_matches.get_one("offset");
            let limit: Option<&i32> = sub_matches.get_one("limit");
            let file_path: Option<&String> = sub_matches.get_one("file-path");
            let pokemon_data = poke_fetch.fetch(limit.copied(), offset.copied()).unwrap();

            // Write to JSON
            let fp = file_path.unwrap();
            pokemon_data.write_json(fp).unwrap()
        }
        _ => unreachable!(),
    }
}
