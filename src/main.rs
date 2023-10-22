use reqwest;

fn fetch_pokemon(pokemon_id: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);
    let res = reqwest::blocking::get(url)?;
    Ok(res)
}

fn main() {
    let pokemon = fetch_pokemon("151").unwrap().text().unwrap();

    println!("{}", pokemon)
}
