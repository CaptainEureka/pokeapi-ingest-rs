use indicatif::ParallelProgressIterator;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use reqwest::{self, blocking::Client};

use crate::models::pokemon::Followable;
use crate::{
    fetcher::errors::FetchError,
    models::pokemon::{PokeApiResponse, Pokemon, PokemonData, PokemonListItem},
};

pub struct PokeFetcher {
    client: Client,
    base_url: String,
}

impl PokeFetcher {
    const BASE_URL: &str = "http://pokeapi.co/api/v2";
    const DEFAULT_PROGRESS_STYLE: &str =
        "{spinner:.green} [{elapsed_precise}] [{bar:100.cyan/blue}] {pos}/{len} ({eta})";

    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: Self::BASE_URL.to_string(),
        }
    }

    pub fn create_progress_style(&self, style_template: Option<&str>) -> ProgressStyle {
        let progress_style = ProgressStyle::default_bar().progress_chars("=>-");
        progress_style
            .template(style_template.unwrap_or(Self::DEFAULT_PROGRESS_STYLE))
            .expect("Unable to create progress bar")
    }

    pub fn create_default_progress_style(&self) -> ProgressStyle {
        self.create_progress_style(None)
    }
}

pub trait FetchPokemon {
    fn fetch_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, FetchError>;
    fn fetch(&self, limit: &i32, offset: &i32) -> Result<Vec<PokemonListItem>, FetchError>;
    fn fetch_all(&self) -> Result<PokemonData, FetchError>;
    fn fetch_with_limit_and_offset(
        &self,
        limit: &i32,
        offset: &i32,
    ) -> Result<PokemonData, FetchError>;
}

impl FetchPokemon for PokeFetcher {
    fn fetch_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, FetchError> {
        self.client
            .get(format!("{}/pokemon/{}", self.base_url, pokemon_id))
            .send()?
            .json()
            .map_err(FetchError::from)
    }

    fn fetch(&self, limit: &i32, offset: &i32) -> Result<Vec<PokemonListItem>, FetchError> {
        let url: String = format!("{}/pokemon", self.base_url);
        let data = self
            .client
            .get(&url)
            .query(&[("limit", limit), ("offset", offset)])
            .send()?
            .json::<PokeApiResponse>()?;

        Ok(data.results)
    }

    fn fetch_with_limit_and_offset(
        &self,
        limit: &i32,
        offset: &i32,
    ) -> Result<PokemonData, FetchError> {
        let pokemon_list = self.fetch(limit, offset)?;

        println!("Fetching {} pokemon", pokemon_list.len());

        let result = pokemon_list
            .par_iter()
            .progress_with_style(self.create_default_progress_style())
            .filter_map(|item| item.follow(&self.client).ok())
            .collect::<Vec<Pokemon>>();

        Ok(PokemonData::new(result))
    }

    fn fetch_all(&self) -> Result<PokemonData, FetchError> {
        self.fetch_with_limit_and_offset(&10000, &0)
    }
}
