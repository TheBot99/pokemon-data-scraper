use reqwest::Error;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
            let id = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replace("data_", "");
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

pub fn flush_moves_json_dir() {
    let _ = std::fs::remove_dir_all("moves_json");
    let _ = std::fs::create_dir("moves_json");
}

pub fn reformat_json_moves() {
    println!("Reformatting JSON files for moves.");
    let dir = Path::new("moves_json");

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let file = fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);

        let mut data: Value = serde_json::from_reader(reader).unwrap();

        if let Some(machines) = data.get_mut("machines").and_then(Value::as_array_mut) {
            let mut new_machines = Map::new();

            for machine in machines.drain(..) {
                if let Value::Object(machine) = machine {
                    if let (Some(machine_url), Some(version_group)) = (
                        machine
                            .get("machine")
                            .and_then(|m| m.get("url"))
                            .and_then(Value::as_str),
                        machine
                            .get("version_group")
                            .and_then(|vg| vg.get("name"))
                            .and_then(Value::as_str),
                    ) {
                        new_machines.insert(
                            version_group.to_string(),
                            Value::String(machine_url.to_string()),
                        );
                    }
                }
            }

            data["machines"] = Value::Object(new_machines);
            data["learned_by_pokemon"] = Value::Null;
            data["names"] = Value::Null;
        }

        if let Some(flavor_text_entries) = data
            .get_mut("flavor_text_entries")
            .and_then(Value::as_array_mut)
        {
            let mut new_flavor_text_entries = Map::new();

            for entry in flavor_text_entries.drain(..) {
                if let Value::Object(entry) = entry {
                    if let (Some(flavor_text), Some(language), Some(version_group)) = (
                        entry.get("flavor_text").and_then(Value::as_str),
                        entry
                            .get("language")
                            .and_then(|l| l.get("name"))
                            .and_then(Value::as_str),
                        entry
                            .get("version_group")
                            .and_then(|vg| vg.get("name"))
                            .and_then(Value::as_str),
                    ) {
                        if language == "en" {
                            new_flavor_text_entries.insert(
                                version_group.to_string(),
                                Value::String(flavor_text.to_string()),
                            );
                        }
                    }
                }
            }

            data["flavor_text_entries"] = Value::Object(new_flavor_text_entries);
        }

        let file = fs::File::create(&path).unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &data).unwrap();
    }

    println!("JSON files for moves reformatted successfully.");
    println!("------------------------------------------------------------");
}
