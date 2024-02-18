use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use reqwest::{self, blocking::Response};
use serde_json;
use std::sync::{Arc, Mutex};

use crate::models::pokemon::{PokeApiResponse, Pokemon, PokemonData};

#[derive(Debug)]
pub enum FetchError {
    Network(reqwest::Error),
    Parse(serde_json::Error),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FetchError::Network(err) => write!(f, "Network error: {}", err),
            FetchError::Parse(err) => write!(f, "Parsing error: {}", err),
        }
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::Network(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Parse(err)
    }
}

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<Response, FetchError>;
}

impl HttpClient for reqwest::blocking::Client {
    fn get(&self, url: &str) -> Result<Response, FetchError> {
        let res = reqwest::blocking::get(url)?;
        Ok(res)
    }
}

pub struct PokeFetcher<T: HttpClient> {
    client: T,
    base_url: String,
}

const BASE_URL: &str = "http://pokeapi.co/api/v2";
const DEFAULT_LIMIT: i32 = 50;

impl<T: HttpClient + std::marker::Sync> PokeFetcher<T> {
    pub fn new(client: T) -> Self {
        Self {
            client,
            base_url: BASE_URL.to_string(),
        }
    }

    fn get_count(&self) -> Result<i32, FetchError> {
        let url = format!("{}/pokemon", self.base_url);

        match self.fetch_pokemon_data(&url, 1, 0) {
            Ok(res) => Ok(res.count),
            Err(err) => {
                println!("An error occurred fetching the total count.");
                Err(FetchError::from(err))
            }
        }
    }

    pub fn fetch(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PokemonData, FetchError> {
        let url = format!("{}/pokemon", self.base_url);

        let pokemon_data = Mutex::new(PokemonData::new(vec![]));

        // Fetch the initial pokemon data
        let iter_generator = generate_limit_offset_pairs(
            self.get_count().unwrap(),
            limit.unwrap_or(DEFAULT_LIMIT),
            offset,
        );

        let multi_progress = Arc::new(MultiProgress::new());

        iter_generator
            .par_iter()
            .for_each_with(multi_progress.clone(), |mp, (limit, offset)| {
                let res = self.fetch_pokemon_data(&url, *limit, *offset).unwrap();

                let progress_bar = mp.add(ProgressBar::new(res.results.len().try_into().unwrap()));

                // Style the progress bar
                progress_bar.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                    .expect("Something")
                    .progress_chars("#>-")
                );

                let pokemon_list: Vec<_> = res
                    .results
                    .par_iter()
                    .filter_map(|item| {
                        let url: &str = &item.url;
                        let pokefetch_result = self.client.get(url).unwrap();
                        let pokemon: Pokemon = pokefetch_result.json().unwrap();

                        // Increment progress bar
                        progress_bar.inc(1);

                        Some(pokemon)
                    })
                    .collect();

                pokemon_data.lock().unwrap().results.extend(pokemon_list);
                progress_bar.finish_with_message("Done fetching...");
            });

        multi_progress.clear().unwrap();

        Ok(pokemon_data.into_inner().unwrap())
    }

    fn fetch_pokemon_data(
        &self,
        url: &str,
        limit: i32,
        offset: i32,
    ) -> Result<PokeApiResponse, reqwest::Error> {
        let mut url = reqwest::Url::parse(url).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("limit", &limit.to_string());
            query_pairs.append_pair("offset", &offset.to_string());
        }
        let res = self.client.get(url.as_ref()).unwrap();

        let pokemon_res: PokeApiResponse = res.json()?;
        Ok(pokemon_res)
    }
}

pub fn generate_limit_offset_pairs(
    total_count: i32,
    limit: i32,
    initial_offset: Option<i32>,
) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();

    let mut offset = initial_offset.unwrap_or(0);

    while offset < total_count {
        pairs.push((limit, offset));
        offset += limit;
    }

    pairs
}

pub fn fetch_pokemon_by_id<T: HttpClient>(
    client: T,
    pokemon_id: &str,
) -> Result<Pokemon, reqwest::Error> {
    let url = format!("{}/pokemon/{}", BASE_URL, pokemon_id);
    let res = client.get(&url).unwrap();

    let pokemon: Pokemon = res.json()?;
    Ok(pokemon)
}
