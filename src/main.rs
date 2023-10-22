mod cli;
mod fetcher;
mod models;

use cli::args::get_args;
use fetcher::pokemon_fetcher::{fetch_pokemon, fetch_pokemon_by_id};
use serde_json;

fn main() {
    let matches = get_args().get_matches();

    match matches.subcommand() {
        Some(("fetch", sub_matches)) => {
            let pokemon_id = sub_matches
                .get_one::<String>("pokemon_id")
                .expect("required");
            let pokemon = fetch_pokemon_by_id(&pokemon_id).unwrap();

            println!("{}", serde_json::to_string_pretty(&pokemon).unwrap())
        }
        Some(("ingest", sub_matches)) => {
            let offset: Option<&i32> = sub_matches.get_one("offset");
            let limit: Option<&i32> = sub_matches.get_one("limit");
            let pokemon_data = fetch_pokemon(limit.copied(), offset.copied()).unwrap();

            println!("{}", serde_json::to_string_pretty(&pokemon_data).unwrap())
        }
        _ => unreachable!(),
    }
}
