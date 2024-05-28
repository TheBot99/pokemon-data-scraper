use std::fs::{self, File};
use std::path::Path;
use serde_json::{Value, Map};
use std::collections::HashMap;

pub fn create_big_json() -> std::io::Result<()> {
    // Directory containing JSON files
    let dir = Path::new("pokemon_json");

    // HashMap to store the big JSON data
    let mut big_json: HashMap<String, Map<String, Value>> = HashMap::new();

    // Iterate over each file in the directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Read file
        let file = fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);

        // Parse the JSON data
        let data: Value = serde_json::from_reader(reader).unwrap();

        // Get the values for the keys "name" and "types"
        if let (Some(name), Some(types)) = (data.get("name"), data.get("types")) {
            // Create a new map for the pokemon data
            let mut pokemon_data = Map::new();
            pokemon_data.insert("name".to_string(), name.clone());
            pokemon_data.insert("types".to_string(), types.clone());

            // Store the pokemon data in the big JSON HashMap
            let id = path.file_stem().unwrap().to_str().unwrap().replace("data_", "");
            big_json.insert(id, pokemon_data);
        }
    }

    // Write the big JSON data to a new JSON file
    let file = File::create("big_json.json")?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &big_json)?;

    Ok(())
}