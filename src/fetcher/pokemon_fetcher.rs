use reqwest::{self, blocking::Response};
use serde_json;

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

impl<T: HttpClient> PokeFetcher<T> {
    pub fn new(client: T) -> Self {
        Self {
            client,
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn fetch(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PokemonData, FetchError> {
        let mut url = Some(String::from(format!("{}/pokemon", self.base_url)));

        let mut pokemon_data = PokemonData::new(vec![]);

        while let Some(next_url) = url {
            let res = self.fetch_pokemon_data(&next_url, limit, offset)?;

            // Iterate through PokemonListItems and fetch the pokemon
            res.results.into_iter().for_each(|item| {
                if let Ok(pokemon) = {
                    let url: &str = &item.url;
                    let res = self.client.get(url).unwrap();

                    let pokemon: Pokemon = res.json().unwrap();
                    Ok::<Pokemon, FetchError>(pokemon)
                } {
                    pokemon_data.results.push(pokemon);
                } else {
                    println!("Failed to fetch pokemon from URL: {}", item.url);
                }
            });
            url = res.next;
        }

        Ok(pokemon_data)
    }

    fn fetch_pokemon_data(
        &self,
        url: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PokeApiResponse, reqwest::Error> {
        let mut url = reqwest::Url::parse(&url).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(limit) = limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(offset) = offset {
                query_pairs.append_pair("offset", &offset.to_string());
            }
        }
        let res = self.client.get(&url.to_string()).unwrap();

        let pokemon_res: PokeApiResponse = res.json()?;
        Ok(pokemon_res)
    }
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
