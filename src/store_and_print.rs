use crate::pokemon_data::PokemonData;
use csv::Writer;
use serde_json::json;
use std::fs::OpenOptions;

pub fn print_data(pokemon_data: &PokemonData) {
    println!(
        "The name of the id:{} is: {}",
        pokemon_data.id, pokemon_data.name
    );
    println!("");
    println!(
        "The weight of {} is: {}",
        pokemon_data.name, pokemon_data.weight
    );
    println!("");
    println!(
        "The height of {} is: {}",
        pokemon_data.name, pokemon_data.height
    );
    println!("");
    println!(
        "The generations of {} are: {:?}",
        pokemon_data.name, pokemon_data.generations
    );
    println!("");
    println!(
        "The abilities of {} are: {:?}",
        pokemon_data.name, pokemon_data.abilities
    );
    println!("");
    println!(
        "The hidden abilities of {} are: {:?}",
        pokemon_data.name, pokemon_data.hidden_abilities
    );
    println!("");
    println!(
        "The types of {} are: {:?}",
        pokemon_data.name, pokemon_data.types
    );
    println!("");
    println!(
        "The base stats of {} are: {:?}",
        pokemon_data.name, pokemon_data.base_stats
    );
    println!("");
    println!(
        "The front sprite default of {} is: {}",
        pokemon_data.name, pokemon_data.front_sprite_default
    );
    println!("");
    println!(
        "The front sprite shiny of {} is: {}",
        pokemon_data.name, pokemon_data.front_sprite_shiny
    );
    println!("");
    println!(
        "The front sprite female default of {} is: {}",
        pokemon_data.name, pokemon_data.front_female_sprite_default
    );
    println!("");
    println!(
        "The front sprite female shiny of {} is: {}",
        pokemon_data.name, pokemon_data.front_female_sprite_shiny
    );
    println!("");
    println!(
        "The pokemon {} has female form: {}",
        pokemon_data.name, pokemon_data.has_female_form
    );
    println!("");
    println!(
        "The moves of {} are: {:?}",
        pokemon_data.name, pokemon_data.moves
    );
    println!("------------------------------------------------------------");
}

pub fn initiate_new_csv_file() {
    let mut writer = Writer::from_path("pokemon_data.csv").unwrap();
    writer.write_record(&[
        "id",
        "name",
        "weight",
        "height",
        "generations",
        "abilities",
        "hidden_abilities",
        "types",
        "base stats",
        "front sprite default",
        "front sprite shiny",
        "front female sprite default",
        "front female sprite shiny",
        "moves",
    ]);
    writer.write_record(&[""]);
    writer.flush().unwrap();

    println!("CSV file created successfully.");
    println!("------------------------------------------------------------");
}

pub fn write_pokemon_data(pokemon_data: &PokemonData) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("pokemon_data.csv")
        .unwrap();

    let moves_json = json!(pokemon_data.moves).to_string();
    let mut writer = csv::Writer::from_writer(file);

    writer.write_record(&[
        &pokemon_data.id.to_string(),
        &pokemon_data.name,
        &pokemon_data.weight.to_string(),
        &pokemon_data.height.to_string(),
        &pokemon_data.generations.join(", "),
        &pokemon_data.abilities.join(", "),
        &pokemon_data.hidden_abilities.join(", "),
        &pokemon_data.types.join(", "),
        &pokemon_data.base_stats.join(", "),
        &pokemon_data.front_sprite_default,
        &pokemon_data.front_sprite_shiny,
        &pokemon_data.front_female_sprite_default,
        &pokemon_data.front_female_sprite_shiny,
        &moves_json,
    ]); // Convert the HashMap to a JSON string
    writer.write_record(&[""]); // This will write a new line
    writer.flush().unwrap();

    println!("Pokemon data written to CSV file successfully.");
    println!("------------------------------------------------------------");
}
