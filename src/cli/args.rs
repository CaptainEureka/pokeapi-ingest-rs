use clap::{Arg, Command};

pub fn get_args() -> Command {
    Command::new("Stockpile CLI - Pokemon Fetcher")
        .bin_name("stockpile")
        .version("0.1.0")
        .author("Your Name <youremail@example.com>")
        .about("Fetches Pokemon data from the PokeAPI")
        .subcommand_required(true)
        .subcommand(
            Command::new("fetch").about("Fetches a Pokemon by ID").arg(
                Arg::new("pokemon_id")
                    .short('p')
                    .long("pokemon-id")
                    .help("ID of the pokemon to fetch"),
            ),
        )
        .subcommand(
            Command::new("ingest")
                .about("Ingests all Pokemon from PokeAPI")
                .arg(
                    Arg::new("limit")
                        .long("limit")
                        .short('l')
                        .default_value("200")
                        .value_parser(clap::value_parser!(i32).range(0..))
                        .action(clap::ArgAction::Set)
                        .help("Pagination limit to use for PokeAPI."),
                )
                .arg(
                    Arg::new("offset")
                        .long("offset")
                        .short('o')
                        .default_value("0")
                        .value_parser(clap::value_parser!(i32).range(0..))
                        .action(clap::ArgAction::Set)
                        .help("Pagination offset to use for PokeAPI."),
                )
                .arg(
                    Arg::new("file-path")
                        .long("file-path")
                        .short('f')
                        .default_value("pokemon_data.json")
                        .action(clap::ArgAction::Set)
                        .help("File output path."),
                ),
        )
}
