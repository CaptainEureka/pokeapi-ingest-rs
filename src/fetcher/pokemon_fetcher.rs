use futures::stream::{self, StreamExt};
use indicatif::ProgressIterator;
use indicatif::ProgressStyle;
use reqwest::Client;

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
    const BASE_URL: &'static str = "http://pokeapi.co/api/v2";
    const DEFAULT_PROGRESS_STYLE: &'static str =
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
    async fn fetch_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, FetchError>;
    async fn fetch(&self, limit: &i32, offset: &i32) -> Result<Vec<PokemonListItem>, FetchError>;
    async fn fetch_all(&self) -> Result<PokemonData, FetchError>;
    async fn fetch_with_limit_and_offset(
        &self,
        limit: &i32,
        offset: &i32,
    ) -> Result<PokemonData, FetchError>;
}

impl FetchPokemon for PokeFetcher {
    async fn fetch_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, FetchError> {
        self.client
            .get(format!("{}/pokemon/{}", self.base_url, pokemon_id))
            .send()
            .await?
            .json()
            .await
            .map_err(FetchError::from)
    }

    async fn fetch(&self, limit: &i32, offset: &i32) -> Result<Vec<PokemonListItem>, FetchError> {
        let url: String = format!("{}/pokemon", self.base_url);
        let data = self
            .client
            .get(&url)
            .query(&[("limit", limit), ("offset", offset)])
            .send()
            .await?
            .json::<PokeApiResponse>()
            .await?;

        Ok(data.results)
    }

    async fn fetch_with_limit_and_offset(
        &self,
        limit: &i32,
        offset: &i32,
    ) -> Result<PokemonData, FetchError> {
        let pokemon_list = self.fetch(limit, offset).await?;

        println!("Fetching {} pokemon", pokemon_list.len());

        let pb = indicatif::ProgressBar::new(pokemon_list.len() as u64)
            .with_style(self.create_default_progress_style());

        let result = stream::iter(pokemon_list)
            .filter_map(|item| {
                pb.inc(1);
                async move {
                    match item.follow(&self.client).await {
                        Ok(pokemon) => Some(pokemon),
                        Err(err) => {
                            eprintln!("Error fetching pokemon: {}", err);
                            None
                        }
                    }
                }
            })
            .collect::<Vec<Pokemon>>()
            .await;

        pb.finish_with_message(format!("Fetched {} pokemon", result.len()));

        Ok(PokemonData::new(result))
    }

    async fn fetch_all(&self) -> Result<PokemonData, FetchError> {
        self.fetch_with_limit_and_offset(&10000, &0).await
    }
}
