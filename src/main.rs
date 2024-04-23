use rustemon::model::pokemon;
use rustemon::pokemon::pokemon::get_by_id;
use std::env;
use tokio;

struct PokemonData {
    id: i64,
    name: String,
    weight: i64,
    height: i64,
    generations: Vec<String>,
    abilities: Vec<String>,
    hidden_abilities: Vec<String>,
    types: Vec<String>,
    base_stats: Vec<String>,
    front_sprite_default: String,
    front_sprite_shiny: String,
    front_female_sprite_default: String,
    front_female_sprite_shiny: String,
}

async fn get_pokemon_by_id(id: i64) -> pokemon::Pokemon {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = get_by_id(id, &rustemon_client).await;
    return pokemon.unwrap();
}

async fn get_pokemon_name(pokemon: pokemon::Pokemon) -> String {
    return pokemon.name;
}

async fn get_pokemon_weight(pokemon: pokemon::Pokemon) -> i64 {
    return pokemon.weight;
}

async fn get_pokemon_height(pokemon: pokemon::Pokemon) -> i64 {
    return pokemon.height;
}

async fn get_pokemon_generations(pokemon: pokemon::Pokemon) -> Vec<String> {
    let generations: std::collections::HashSet<String> = pokemon
        .moves
        .iter()
        .flat_map(|mv| mv.version_group_details.iter())
        .map(|detail| {
            detail
                .version_group
                .name
                .split("-")
                .next()
                .unwrap()
                .to_string()
        })
        .collect();
    let generations_list: Vec<String> = generations.into_iter().map(|s| s.to_string()).collect();
    return generations_list;
}

async fn get_abilities(pokemon: pokemon::Pokemon) -> Vec<String> {
    let abilities: Vec<String> = pokemon
        .abilities
        .iter()
        .filter(|abiliti| !abiliti.is_hidden)
        .map(|ability| ability.ability.name.to_string())
        .collect();
    return abilities;
}

async fn get_hidden_abilities(pokemon: pokemon::Pokemon) -> Vec<String> {
    let hidden_abilities: Vec<String> = pokemon
        .abilities
        .iter()
        .filter(|ability| ability.is_hidden)
        .map(|ability| ability.ability.name.to_string())
        .collect();
    return hidden_abilities;
}

async fn get_types(pokemon: pokemon::Pokemon) -> Vec<String> {
    let types: Vec<String> = pokemon
        .types
        .iter()
        .map(|type_| type_.type_.name.to_string())
        .collect();
    return types;
}

async fn get_base_stats(pokemon: pokemon::Pokemon) -> Vec<String> {
    let base_stats: Vec<String> = pokemon
        .stats
        .iter()
        .map(|stat| stat.base_stat.to_string())
        .collect();
    return base_stats;
}

async fn get_front_sprite_default(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_sprite_default = pokemon.sprites.front_default;
    return front_sprite_default;
}

async fn get_front_sprite_shiny(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_sprite_shiny = pokemon.sprites.front_shiny;
    return front_sprite_shiny;
}

async fn get_front_female_sprite_default(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_female_sprite_default = pokemon.sprites.front_female;
    if front_female_sprite_default == None {
        return Some("No female sprite default found.".to_string());
    }
    return front_female_sprite_default;
}

async fn get_front_female_sprite_shiny(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_female_sprite_shiny = pokemon.sprites.front_shiny_female;
    if front_female_sprite_shiny == None {
        return Some("No female sprite shiny found.".to_string());
    }
    return front_female_sprite_shiny;
}

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
    };
}

fn print_data(pokemon_data: PokemonData) {
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
    println!("------------------------------------------------------------");
}

fn main() {
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
        print_data(pokemon_data);
        id += 1;
    }
}
