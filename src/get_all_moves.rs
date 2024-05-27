use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use serde_json::Value;

pub fn main() {
    let mut unique_moves = HashSet::new();

    // Iterate through each file in the "pokemon_json" directory
    for entry in fs::read_dir("pokemon_json").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file = File::open(&path).unwrap();
        let json: Value = serde_json::from_reader(file).unwrap();

        // Function to extract moves from the given section
        let mut extract_moves = |section: &Value| {
            if let Some(arr) = section.as_array() {
                for move_pair in arr {
                    if let Some(move_name) = move_pair[0].as_str() {
                        unique_moves.insert(move_name.to_string());
                    }
                }
            }
        };

        // Extract moves from different categories
        if let Some(moves) = json["moves"].as_object() {
            for (_, move_categories) in moves {
                if let Some(categories) = move_categories.as_object() {
                    for (_, move_list) in categories {
                        extract_moves(move_list);
                    }
                }
            }
        }
    }

    // Create a JSON array from the HashSet
    let unique_moves_vec: Vec<String> = unique_moves.iter().cloned().collect();
    let unique_moves_json = serde_json::to_string(&unique_moves_vec).unwrap();

    // Write the unique moves to a new JSON file
    let output_path = Path::new("moves_json/unique_moves.json");
    let mut output_file = File::create(&output_path).unwrap();
    output_file.write_all(unique_moves_json.as_bytes()).unwrap();

    // Print the number of unique moves
    println!("Number of unique moves: {}", unique_moves.len());
    println!("Unique moves have been written to unique_moves.json");

    println!("------------------------------------------------------------");
}
