use rustemon::model::pokemon;
use rustemon::pokemon::pokemon::get_by_id;
use std::collections::HashMap;
use std::collections::HashSet;

pub async fn get_pokemon_by_id(id: i64) -> pokemon::Pokemon {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = get_by_id(id, &rustemon_client).await;
    return pokemon.unwrap();
}

pub async fn get_pokemon_name(pokemon: pokemon::Pokemon) -> String {
    return pokemon.name;
}

pub async fn get_pokemon_weight(pokemon: pokemon::Pokemon) -> i64 {
    return pokemon.weight;
}

pub async fn get_pokemon_height(pokemon: pokemon::Pokemon) -> i64 {
    return pokemon.height;
}

pub async fn get_pokemon_generations(pokemon: pokemon::Pokemon) -> Vec<String> {
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

pub async fn get_abilities(pokemon: pokemon::Pokemon) -> Vec<String> {
    let abilities: Vec<String> = pokemon
        .abilities
        .iter()
        .filter(|abiliti| !abiliti.is_hidden)
        .map(|ability| ability.ability.name.to_string())
        .collect();
    return abilities;
}

pub async fn get_hidden_abilities(pokemon: pokemon::Pokemon) -> Vec<String> {
    let hidden_abilities: Vec<String> = pokemon
        .abilities
        .iter()
        .filter(|ability| ability.is_hidden)
        .map(|ability| ability.ability.name.to_string())
        .collect();
    return hidden_abilities;
}

pub async fn get_types(pokemon: pokemon::Pokemon) -> Vec<String> {
    let types: Vec<String> = pokemon
        .types
        .iter()
        .map(|type_| type_.type_.name.to_string())
        .collect();
    return types;
}

pub async fn get_base_stats(pokemon: pokemon::Pokemon) -> Vec<String> {
    let base_stats: Vec<String> = pokemon
        .stats
        .iter()
        .map(|stat| stat.base_stat.to_string())
        .collect();
    return base_stats;
}

pub async fn get_front_sprite_default(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_sprite_default = pokemon.sprites.front_default;
    return front_sprite_default;
}

pub async fn get_front_sprite_shiny(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_sprite_shiny = pokemon.sprites.front_shiny;
    return front_sprite_shiny;
}

pub async fn get_front_female_sprite_default(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_female_sprite_default = pokemon.sprites.front_female;
    if front_female_sprite_default == None {
        return Some("No female sprite default found.".to_string());
    }
    return front_female_sprite_default;
}

pub async fn get_front_female_sprite_shiny(pokemon: pokemon::Pokemon) -> Option<String> {
    let front_female_sprite_shiny = pokemon.sprites.front_shiny_female;
    if front_female_sprite_shiny == None {
        return Some("No female sprite shiny found.".to_string());
    }
    return front_female_sprite_shiny;
}

pub fn get_pokemon_moves(
    pokemon: pokemon::Pokemon,
) -> HashMap<String, HashMap<String, HashSet<(String, String)>>> {
    let mut moves_by_generation: HashMap<String, HashMap<String, HashSet<(String, String)>>> =
        HashMap::new();

    for move_ in &pokemon.moves {
        for detail in &move_.version_group_details {
            let version_group = &detail.version_group.name;
            let generation = match version_group.as_str() {
                "red-blue" | "yellow" => "generation-i",
                "gold-silver" | "crystal" => "generation-ii",
                "ruby-sapphire" | "emerald" | "firered-leafgreen" => "generation-iii",
                "diamond-pearl" | "platinum" | "heartgold-soulsilver" => "generation-iv",
                "black-white" | "black-2-white-2" => "generation-v",
                "x-y" | "omega-ruby-alpha-sapphire" => "generation-vi",
                "sun-moon" | "ultra-sun-ultra-moon" => "generation-vii",
                "lets-go" | "sword-shield" => "generation-viii",
                _ => "unknown",
            }
            .to_string();

            let generation_moves = moves_by_generation.entry(generation).or_insert_with(|| {
                let mut methods: HashMap<String, HashSet<(String, String)>> = HashMap::new();
                methods.insert("level-up".to_string(), HashSet::new());
                methods.insert("machine".to_string(), HashSet::new());
                methods.insert("egg".to_string(), HashSet::new());
                methods.insert("tutor".to_string(), HashSet::new());
                methods
            });

            let method = &detail.move_learn_method.name;
            if let Some(moves) = generation_moves.get_mut(method) {
                if method == "level-up" {
                    moves.insert((
                        move_.move_.name.to_string(),
                        detail.level_learned_at.to_string(),
                    ));
                } else {
                    moves.insert((move_.move_.name.to_string(), "".to_string()));
                }
            }
        }
    }

    moves_by_generation
}
