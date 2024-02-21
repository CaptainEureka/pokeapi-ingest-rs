use indicatif::MultiProgress;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use reqwest::{self, blocking::Client};
use std::sync::{Arc, Mutex};

use crate::{
    fetcher::{errors::FetchError, progress::ProgressBarHandler},
    models::pokemon::{PokeApiResponse, Pokemon, PokemonData, PokemonListItem},
};

use super::progress::DEFAULT_PROGRESS_STYLE;

pub struct PokeFetcher {
    client: Client,
    base_url: String,
}

pub trait FetchAllPokemon {
    fn total(&self) -> Result<i32, FetchError>;
    fn fetch(&self, limit: i32, offset: i32) -> Result<PokemonData, FetchError>;
    fn fetch_pokemon_data(
        &self,
        url: &str,
        limit: i32,
        offset: i32,
    ) -> Result<PokeApiResponse, FetchError>;
}

pub trait FetchPokemon {
    fn fetch_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, FetchError>;
}

impl FetchPokemon for PokeFetcher {
    fn fetch_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, FetchError> {
        self.client
            .get(format!("{}/pokemon/{}", self.base_url, pokemon_id))
            .send()?
            .json()
            .map_err(FetchError::from)
    }
}

impl PokeFetcher {
    const BASE_URL: &str = "http://pokeapi.co/api/v2";

    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: Self::BASE_URL.to_string(),
        }
    }
}

impl FetchAllPokemon for PokeFetcher {
    fn total(&self) -> Result<i32, FetchError> {
        Ok(self
            .fetch_pokemon_data(&(format!("{}/pokemon", self.base_url)), 1, 0)?
            .count)
    }

    fn fetch(&self, limit: i32, offset: i32) -> Result<PokemonData, FetchError> {
        let url = format!("{}/pokemon", self.base_url);

        let pokemon_data = Mutex::new(PokemonData::new(vec![]));

        // Fetch the initial pokemon data
        let iter_generator = generate_limit_offset_pairs(self.total()?, limit, offset);

        let multi_progress = Arc::new(MultiProgress::new());

        iter_generator
            .par_iter()
            .for_each_with(multi_progress.clone(), |mp, (limit, offset)| {
                // Get the initial pokemon data
                let res = match self.fetch_pokemon_data(&url, *limit, *offset) {
                    Ok(res) => res,
                    Err(err) => {
                        eprintln!("Error fetching pokemon data: {}", err);
                        return;
                    }
                };

                // Setup the Progress Bar
                let progress_bar_handler = ProgressBarHandler::new(res.results.len() as u64);
                let pb = mp.add(progress_bar_handler.progress_bar);

                // Iterator through the pokemon data, for each pokemon_url fetch that data
                let pokemon_list: Vec<Pokemon> = res
                    .results
                    .par_iter()
                    .filter_map(|item| {
                        // Increment progress bar
                        pb.inc(1);

                        self.client
                            .get(&item.url)
                            .send()
                            .ok()
                            .and_then(|res| res.json().ok())
                    })
                    .collect();

                match pokemon_data.lock() {
                    Ok(mut data) => {
                        data.results.extend(pokemon_list);
                    }
                    Err(err) => {
                        eprintln!("Error writing to pokemon data: {}", err);
                    }
                }
                pb.finish_with_message("Done fetching...");
            });

        multi_progress.clear().map_err(FetchError::from)?;

        pokemon_data.into_inner().map_err(FetchError::from)
    }

    fn fetch_pokemon_data(
        &self,
        url: &str,
        limit: i32,
        offset: i32,
    ) -> Result<PokeApiResponse, FetchError> {
        self.client
            .get(url)
            .query(&[("limit", limit), ("offset", offset)])
            .send()?
            .json::<PokeApiResponse>()
            .map_err(FetchError::from)
    }
}

pub trait NewFetch {
    fn fetch_all(&self) -> Result<(i32, Vec<PokemonListItem>), FetchError>;
    fn fetch_from_all(&self) -> Result<PokemonData, FetchError>;
}

impl NewFetch for PokeFetcher {
    fn fetch_all(&self) -> Result<(i32, Vec<PokemonListItem>), FetchError> {
        let url: String = format!("{}/pokemon", self.base_url);
        let data = self
            .client
            .get(&url)
            .query(&[("limit", "10000"), ("offset", "0")])
            .send()?
            .json::<PokeApiResponse>()?;

        Ok((data.count, data.results))
    }

    fn fetch_from_all(&self) -> Result<PokemonData, FetchError> {
        let (_total, pokemon_list) = self.fetch_all()?;

        let style = ProgressStyle::default_bar()
            .template(DEFAULT_PROGRESS_STYLE)
            .expect("Unable to create progress bar")
            .progress_chars("=>-");

        let result = pokemon_list
            .par_iter()
            .progress_with_style(style)
            .filter_map(|item| {
                // Get the individual pokemon data
                self.client
                    .get(&item.url)
                    .send()
                    .and_then(|res| res.json::<Pokemon>())
                    .ok()
            })
            .collect::<Vec<Pokemon>>();

        Ok(PokemonData::new(result))
    }
}

pub fn generate_limit_offset_pairs(total_count: i32, limit: i32, offset: i32) -> Vec<(i32, i32)> {
    (offset..total_count)
        .step_by(limit as usize)
        .map(|offset| (limit, offset))
        .collect()
}
