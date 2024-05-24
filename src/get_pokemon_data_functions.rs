use rustemon::model::evolution;
use rustemon::model::pokemon;
use rustemon::model::pokemon::PokemonSpecies;
use rustemon::pokemon::pokemon::get_by_id;
use std::collections::HashMap;
use std::collections::HashSet;

pub async fn get_pokemon_species_by_id(id: i64) -> PokemonSpecies {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon_species = rustemon::pokemon::pokemon_species::get_by_id(id, &rustemon_client).await;
    return pokemon_species.unwrap();
}

pub async fn get_evolution_chain(id: i64) -> evolution::EvolutionChain {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let evolution_chain =
        rustemon::evolution::evolution_chain::get_by_id(id, &rustemon_client).await;
    return evolution_chain.unwrap();
}

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

pub async fn get_hidden_ability(pokemon: pokemon::Pokemon) -> String {
    let hidden_abilities: String = pokemon
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
                "scarlet-violet" => "generation-ix",
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
                } else if method == "machine" {
                    let move_id = detail.move_learn_method.url.split("/").last().unwrap();
                    moves.insert((move_.move_.name.to_string(), "".to_string()));
                } else {
                    moves.insert((move_.move_.name.to_string(), "".to_string()));
                }
            }
        }
    }

    moves_by_generation
}

pub async fn get_evs(pokemon: pokemon::Pokemon) -> Vec<String> {
    let evs: Vec<String> = pokemon
        .stats
        .iter()
        .map(|stat| stat.effort.to_string())
        .collect();
    return evs;
}

pub async fn get_evolution_chain_id(pokemon: pokemon::Pokemon) -> i64 {
    let species = tokio::runtime::Runtime::new().unwrap().block_on(async {
        get_pokemon_species_by_id(
            pokemon
                .species
                .url
                .split("/")
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        )
        .await
    });
    let url = species.evolution_chain.unwrap().url;
    let url_parts: Vec<&str> = url.split('/').collect();
    let evolution_chain_id: i64 = url_parts[url_parts.len() - 2].parse().unwrap();
    evolution_chain_id
}

pub async fn get_evolution_chain_details(
    pokemon: pokemon::Pokemon,
) -> Vec<(String, String, String)> {
    let evolution_chain_id = tokio::task::spawn_blocking(|| get_evolution_chain_id(pokemon))
        .await
        .unwrap();
    let evolution_chain = get_evolution_chain(evolution_chain_id.await).await;
    let mut evolution_details: Vec<(String, String, String)> = Vec::new();

    let mut current_evolution = &evolution_chain.chain;

    while let Some(evolution) = &current_evolution.evolves_to.first() {
        let species_name = &current_evolution.species.name;
        let evolves_to_name = &evolution.species.name;
        let evolution_trigger = &evolution.evolution_details.first().unwrap().trigger.name;

        evolution_details.push((
            species_name.clone(),
            evolves_to_name.clone(),
            evolution_trigger.clone(),
        ));

        current_evolution = evolution;
    }

    evolution_details
}
