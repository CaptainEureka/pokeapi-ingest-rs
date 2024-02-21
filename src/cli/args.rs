use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, long_about = None, subcommand_required = true)]
#[command(name = "stockpile", about = "Fetches Pokemon data from the PokeAPI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true, about = "Fetches a Pokemon by ID")]
    Fetch {
        #[arg(short, long, help = "ID of the pokemon to fetch")]
        pokemon_id: String,
    },
    #[command(about = "Ingests all Pokemon from PokeAPI")]
    Ingest {
        #[arg(
            long,
            short,
            default_value = "200",
            required = false,
            help = "Pagination limit to use for PokeAPI."
        )]
        /// The limit for the number of items to in a page from PokeAPI.
        /// It should be a positive integer.
        limit: i32,
        #[arg(
            long,
            short,
            default_value = "0",
            required = false,
            help = "Pagination offset to use for PokeAPI."
        )]
        /// The offset value for retrieving data from the PokeAPI.
        /// This value determines the starting point of the data to be retrieved.
        offset: i32,
        #[arg(
            long,
            short,
            default_value = "pokemon_data.json",
            required = false,
            help = "File output path."
        )]
        /// The file path of the input file.
        file_path: String,
    },
}
