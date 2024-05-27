use std::fs::File;
use std::io::Write;
use reqwest::Error;
use serde_json::Value;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    // Initial ID
    let mut id = 1;
    // Maximum ID
    let max_id = 919;

    while id <= max_id {
    // URL to fetch data from
        let url = format!("https://pokeapi.co/api/v2/move/{}/", id);

        // Send GET request
        let response = reqwest::get(&url).await?;

        // Parse JSON data
        let data: Value = response.json().await?;

        // Convert JSON data to string
        let data_string = data.to_string();

        // Write data to a file
        let mut file = File::create(format!("moves_json/{}.json", id)).unwrap();
        file.write_all(data_string.as_bytes()).unwrap();

        // Increment ID
        id += 1;
    }

    Ok(())
}