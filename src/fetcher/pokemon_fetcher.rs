use reqwest;

use crate::models::pokemon::{PokeApiResponse, Pokemon, PokemonData};

pub fn fetch_pokemon(
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<PokemonData, reqwest::Error> {
    let mut url = Some(String::from("https://pokeapi.co/api/v2/pokemon"));

    let mut pokemon_data = PokemonData::new(vec![]);

    while let Some(next_url) = url {
        let res = fetch_pokemon_data(&next_url, limit, offset)?;

        // Iterate through PokemonListItems and fetch the pokemon
        res.results.into_iter().for_each(|item| {
            if let Ok(pokemon) = fetch_pokemon_by_url(&item.url) {
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
    let res = reqwest::blocking::get(url)?;

    let pokemon_res: PokeApiResponse = res.json()?;
    Ok(pokemon_res)
}

fn fetch_pokemon_by_url(url: &str) -> Result<Pokemon, reqwest::Error> {
    let res = reqwest::blocking::get(url)?;

    let pokemon: Pokemon = res.json()?;
    Ok(pokemon)
}

pub fn fetch_pokemon_by_id(pokemon_id: &str) -> Result<Pokemon, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);
    let res = reqwest::blocking::get(url)?;

    let pokemon: Pokemon = res.json()?;
    Ok(pokemon)
}
