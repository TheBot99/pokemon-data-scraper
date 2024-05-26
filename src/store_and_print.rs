use crate::pokemon_data::PokemonData;
use csv::Writer;
use serde_json;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write; // Import the std::io::Write trait

fn get_moves_by_generation_and_type<'a>(
    pokemon_data: &'a PokemonData,
    generation: &'a str,
    move_type: &'a str,
) -> HashSet<(String, String)> {
    let empty_set = HashSet::new();

    match pokemon_data.moves.get_key_value(generation) {
        Some((_, v)) => match v.get_key_value(move_type) {
            Some((_, v)) => v.clone(),
            None => empty_set,
        },
        None => empty_set,
    }
}

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
        pokemon_data.name, pokemon_data.hidden_ability
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
        "The evs of {} are: {:?}",
        pokemon_data.name, pokemon_data.evs
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
        "The evolution chain ID of {} is: {:?}",
        pokemon_data.name, pokemon_data.evolution_chain_id
    );
    println!("");
    println!(
        "The generation-i egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-i", "egg")
    );
    println!("");
    println!(
        "The generation-i level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-i", "level-up")
    );
    println!("");
    println!(
        "The generation-i machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-i", "machine")
    );
    println!("");
    println!(
        "The generation-i tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-i", "tutor")
    );
    println!("");
    println!(
        "The generation-ii egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-ii", "egg")
    );
    println!("");
    println!(
        "The generation-ii level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-ii", "level-up")
    );
    println!("");
    println!(
        "The generation-ii machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-ii", "machine")
    );
    println!("");
    println!(
        "The generation-ii tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-ii", "tutor")
    );
    println!("");
    println!(
        "The generation-iii egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iii", "egg")
    );
    println!("");
    println!(
        "The generation-iii level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iii", "level-up")
    );
    println!("");
    println!(
        "The generation-iii machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iii", "machine")
    );
    println!("");
    println!(
        "The generation-iii tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iii", "tutor")
    );
    println!("");
    println!(
        "The generation-iv egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iv", "egg")
    );
    println!("");
    println!(
        "The generation-iv level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iv", "level-up")
    );
    println!("");
    println!(
        "The generation-iv machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iv", "machine")
    );
    println!("");
    println!(
        "The generation-iv tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-iv", "tutor")
    );
    println!("");
    println!(
        "The generation-v egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-v", "egg")
    );
    println!("");
    println!(
        "The generation-v level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-v", "level-up")
    );
    println!("");
    println!(
        "The generation-v machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-v", "machine")
    );
    println!("");
    println!(
        "The generation-v tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-v", "tutor")
    );
    println!("");
    println!(
        "The generation-vi egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vi", "egg")
    );
    println!("");
    println!(
        "The generation-vi level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vi", "level-up")
    );
    println!("");
    println!(
        "The generation-vi machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vi", "machine")
    );
    println!("");
    println!(
        "The generation-vi tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vi", "tutor")
    );
    println!("");
    println!(
        "The generation-vii egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vii", "egg")
    );
    println!("");
    println!(
        "The generation-vii level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vii", "level-up")
    );
    println!("");
    println!(
        "The generation-vii machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vii", "machine")
    );
    println!("");
    println!(
        "The generation-vii tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-vii", "tutor")
    );
    println!("");
    println!(
        "The generation-viii egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-viii", "egg")
    );
    println!("");
    println!(
        "The generation-viii level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-viii", "level-up")
    );
    println!("");
    println!(
        "The generation-viii machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-viii", "machine")
    );
    println!("");
    println!(
        "The generation-viii tutor moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-viii", "tutor")
    );
    println!("");
    println!(
        "The generation-ix egg moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-ix", "egg")
    );
    println!("");
    println!(
        "The generation-ix level-up moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-ix", "level-up")
    );
    println!("");
    println!(
        "The generation-ix machine moves of {} are: {:?}",
        pokemon_data.name,
        get_moves_by_generation_and_type(pokemon_data, "generation-ix", "machine")
    );

    println!("------------------------------------------------------------");
    // println!("");
    // println!(
    //     "The evolution chain of {} is: {:?}",
    //     pokemon_data.name, pokemon_data.evolution_chain
    // );
}

pub fn initiate_new_csv_file(pokemon_data: &PokemonData) {
    let mut writer = Writer::from_path(&format!("pokemon_csv/{}.csv", pokemon_data.name)).unwrap();
    let _ = writer.write_record(&[
        "id",
        "name",
        "weight",
        "height",
        "generations",
        "ability 1",
        "ability 2",
        "hidden ability",
        "type 1",
        "type 2",
        "hp",
        "attack",
        "defense",
        "special attack",
        "special defense",
        "speed",
        "ev hp",
        "ev attack",
        "ev defense",
        "ev special attack",
        "ev special defense",
        "ev speed",
        "front sprite default",
        "front sprite shiny",
        "front female sprite default",
        "front female sprite shiny",
        "evolution_chain_id",
        "Gen 1 egg",
        "Gen 1 level up",
        "Gen 1 machine",
        "Gen 1 tutor",
        "Gen 2 egg",
        "Gen 2 level up",
        "Gen 2 machine",
        "Gen 2 tutor",
        "Gen 3 egg",
        "Gen 3 level up",
        "Gen 3 machine",
        "Gen 3 tutor",
        "Gen 4 egg",
        "Gen 4 level up",
        "Gen 4 machine",
        "Gen 4 tutor",
        "Gen 5 egg",
        "Gen 5 level up",
        "Gen 5 machine",
        "Gen 5 tutor",
        "Gen 6 egg",
        "Gen 6 level up",
        "Gen 6 machine",
        "Gen 6 tutor",
        "Gen 7 egg",
        "Gen 7 level up",
        "Gen 7 machine",
        "Gen 7 tutor",
        "Gen 8 egg",
        "Gen 8 level up",
        "Gen 8 machine",
        "Gen 8 tutor",
        "Gen 9 egg",
        "Gen 9 level up",
        "Gen 9 machine",
    ]);
    let _ = writer.write_record(&[""]);
    writer.flush().unwrap();

    println!("CSV file created successfully.");
    println!("------------------------------------------------------------");
}

pub fn write_pokemon_data_csv(pokemon_data: &PokemonData) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&format!("pokemon_csv/{}.csv", pokemon_data.name))
        .unwrap();

    let binding = "".to_string();
    let zero = "0".to_string();

    let mut writer = csv::Writer::from_writer(file);

    let _ = writer.write_record(&[
        &pokemon_data.id.to_string(),
        &pokemon_data.name,
        &pokemon_data.weight.to_string(),
        &pokemon_data.height.to_string(),
        &pokemon_data.generations.join(", "),
        &pokemon_data.abilities.get(0).unwrap_or(&binding),
        &pokemon_data.abilities.get(1).unwrap_or(&binding),
        &pokemon_data.hidden_ability,
        &pokemon_data.types.get(0).unwrap_or(&binding),
        &pokemon_data.types.get(1).unwrap_or(&binding),
        &pokemon_data.base_stats.get(0).unwrap_or(&binding),
        &pokemon_data.base_stats.get(1).unwrap_or(&binding),
        &pokemon_data.base_stats.get(2).unwrap_or(&binding),
        &pokemon_data.base_stats.get(3).unwrap_or(&binding),
        &pokemon_data.base_stats.get(4).unwrap_or(&binding),
        &pokemon_data.base_stats.get(5).unwrap_or(&binding),
        &pokemon_data.evs.get(0).unwrap_or(&zero),
        &pokemon_data.evs.get(1).unwrap_or(&zero),
        &pokemon_data.evs.get(2).unwrap_or(&zero),
        &pokemon_data.evs.get(3).unwrap_or(&zero),
        &pokemon_data.evs.get(4).unwrap_or(&zero),
        &pokemon_data.evs.get(5).unwrap_or(&zero),
        &pokemon_data.front_sprite_default,
        &pokemon_data.front_sprite_shiny,
        &pokemon_data.front_female_sprite_default,
        &pokemon_data.front_female_sprite_shiny,
        &pokemon_data.evolution_chain_id.to_string(),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-i", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-i", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-i", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-i", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-ii", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-ii", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-ii", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-ii", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iii", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iii", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iii", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iii", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iv", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iv", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iv", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-iv", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-v", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-v", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-v", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-v", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vi", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vi", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vi", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vi", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vii", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vii", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vii", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-vii", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-viii", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-viii", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-viii", "machine")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-viii", "tutor")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-ix", "egg")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-ix", "level-up")
        ),
        &format!(
            "{:?}",
            get_moves_by_generation_and_type(pokemon_data, "generation-ix", "machine")
        ),
    ]); // Convert the HashMap to a JSON string
    let _ = writer.write_record(&[""]); // This will write a new line
    writer.flush().unwrap();

    println!("Pokemon data written to CSV file successfully.");
    println!("------------------------------------------------------------");
}

pub fn write_pokemon_data_json(pokemon_data: &PokemonData) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&format!("pokemon_json/{}.json", pokemon_data.name))
        .unwrap();

    let json = serde_json::json!({
        "id": pokemon_data.id,
        "name": pokemon_data.name,
        "weight": pokemon_data.weight,
        "height": pokemon_data.height,
        "generations": pokemon_data.generations,
        "abilities": pokemon_data.abilities,
        "hidden_ability": pokemon_data.hidden_ability,
        "types": pokemon_data.types,
        "base_stats": pokemon_data.base_stats,
        "evs": pokemon_data.evs,
        "front_sprite_default": pokemon_data.front_sprite_default,
        "front_sprite_shiny": pokemon_data.front_sprite_shiny,
        "front_female_sprite_default": pokemon_data.front_female_sprite_default,
        "front_female_sprite_shiny": pokemon_data.front_female_sprite_shiny,
        "evolution_chain_id" : pokemon_data.evolution_chain_id,
        "moves": pokemon_data.moves,
    });

    let mut writer = std::io::BufWriter::new(file);
    let formatted_json = serde_json::to_string_pretty(&json).unwrap();
    writer.write_all(formatted_json.as_bytes()).unwrap();

    println!("Pokemon data written to JSON file successfully.");
    println!("------------------------------------------------------------");
}

pub fn flush_json_dir() {
    let _ = std::fs::remove_dir_all("pokemon_json");
    let _ = std::fs::create_dir("pokemon_json");

    println!("JSON directory flushed successfully.");
    println!("------------------------------------------------------------");
}

pub fn flush_csv_dir() {
    let _ = std::fs::remove_dir_all("pokemon_csv");
    let _ = std::fs::create_dir("pokemon_csv");

    println!("CSV directory flushed successfully.");
    println!("------------------------------------------------------------");
}
