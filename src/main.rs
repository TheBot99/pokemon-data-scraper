mod get_pokemon_data_functions;
use get_pokemon_data_functions::{
    get_abilities, get_base_stats, get_front_female_sprite_default, get_front_female_sprite_shiny,
    get_front_sprite_default, get_front_sprite_shiny, get_hidden_abilities, get_pokemon_by_id,
    get_pokemon_generations, get_pokemon_height, get_pokemon_name, get_pokemon_weight, get_types,
};
mod store_and_print;
use std::env;
use tokio;
mod pokemon_data;
use pokemon_data::PokemonData;

fn get_data(id: i64) -> PokemonData {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    let pokemon = runtime.block_on(async { get_pokemon_by_id(id).await });

    let pokemon_name = runtime.block_on(async { get_pokemon_name(pokemon.clone()).await });

    let pokemon_weight = runtime.block_on(async { get_pokemon_weight(pokemon.clone()).await });

    let pokemon_height = runtime.block_on(async { get_pokemon_height(pokemon.clone()).await });

    let pokemon_generations =
        runtime.block_on(async { get_pokemon_generations(pokemon.clone()).await });

    let pokemon_abilities = runtime.block_on(async { get_abilities(pokemon.clone()).await });

    let pokemon_hidden_abilities =
        runtime.block_on(async { get_hidden_abilities(pokemon.clone()).await });

    let pokemon_types = runtime.block_on(async { get_types(pokemon.clone()).await });

    let pokemon_base_stats = runtime.block_on(async { get_base_stats(pokemon.clone()).await });

    let pokemon_front_sprite_default =
        runtime.block_on(async { get_front_sprite_default(pokemon.clone()).await });

    let pokemon_front_sprite_shiny =
        runtime.block_on(async { get_front_sprite_shiny(pokemon.clone()).await });

    let pokemon_front_female_sprite_default =
        runtime.block_on(async { get_front_female_sprite_default(pokemon.clone()).await });

    let pokemon_front_female_sprite_shiny =
        runtime.block_on(async { get_front_female_sprite_shiny(pokemon.clone()).await });
    let mut has_pokemon_female_form = true;
    if pokemon_front_female_sprite_default == Some("No female sprite default found.".to_string()) {
        has_pokemon_female_form = false;
    }

    let moves = get_pokemon_data_functions::get_pokemon_moves(pokemon.clone());

    return PokemonData {
        id: id,
        name: pokemon_name,
        weight: pokemon_weight,
        height: pokemon_height,
        generations: pokemon_generations,
        abilities: pokemon_abilities,
        hidden_abilities: pokemon_hidden_abilities,
        types: pokemon_types,
        base_stats: pokemon_base_stats,
        front_sprite_default: pokemon_front_sprite_default.expect("No front sprite default found."),
        front_sprite_shiny: pokemon_front_sprite_shiny.expect("No front sprite shiny found."),
        front_female_sprite_default: pokemon_front_female_sprite_default
            .expect("No front female sprite default found."),
        front_female_sprite_shiny: pokemon_front_female_sprite_shiny
            .expect("No front female sprite shiny found."),
        has_female_form: has_pokemon_female_form,
        moves: moves,
    };
}

fn main() {
    store_and_print::initiate_new_csv_file();
    let args: Vec<String> = env::args().collect();
    let mut id: i64 = 1;
    let mut max_id: i64 = 1025;

    for (index, arg) in args.iter().enumerate() {
        if arg == "--min" {
            if let Some(min) = args.get(index + 1) {
                let min_id = min.parse::<i64>().unwrap_or(1);
                id = min_id.max(1).min(1025);
            }
        } else if arg == "--max" {
            if let Some(max) = args.get(index + 1) {
                let max_id_value = max.parse::<i64>().unwrap_or(1025);
                max_id = max_id_value.max(id).min(1025);
            }
        }
    }

    while id <= max_id {
        let pokemon_data = get_data(id);
        store_and_print::write_pokemon_data(&pokemon_data);
        store_and_print::print_data(&pokemon_data);
        id += 1;
    }
}
