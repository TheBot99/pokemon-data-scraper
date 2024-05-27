use reqwest::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    println!("Fetching data for all moves.");
    println!("This may take a while.");
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


    println!("Data for all moves fetched successfully.");
    println!("------------------------------------------------------------");

    Ok(())
}


pub fn make_name_id_index() -> std::io::Result<()> {
    println!("Creating name-id index for moves.");
    // Directory containing JSON files
    let dir = Path::new("moves_json");

    // HashMap to store name-id pairs
    let mut name_id_index: HashMap<String, String> = HashMap::new();

    // Iterate over each file in the directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Read the file
        let file = fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);

        // Parse the JSON data
        let data: Value = serde_json::from_reader(reader).unwrap();

        // Get the value for the key "name"
        if let Some(name) = data.get("name").and_then(Value::as_str) {
            // Store the name-id pair in the HashMap
            let id = path.file_stem().unwrap().to_str().unwrap().replace("data_", "");
            name_id_index.insert(name.to_string(), id);
        }
    }

    // Write the name-id pairs to a new JSON file
    let file = File::create("moves_json/name_id_index.json")?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &name_id_index)?;

    println!("Name-id index created successfully.");
    println!("------------------------------------------------------------");

    Ok(())
}
