use rustemon::model::evolution::EvolutionChain;
use rustemon::model::machines::Machine;
use rustemon::model::moves::Move;
use rustemon::model::pokemon;
use rustemon::model::pokemon::PokemonSpecies;
use rustemon::model::resource::ApiResource;
use rustemon::pokemon::pokemon::get_by_id;
use std::collections::HashMap;
use std::collections::HashSet;

pub async fn get_pokemon_species_by_id(id: i64) -> PokemonSpecies {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon_species = rustemon::pokemon::pokemon_species::get_by_id(id, &rustemon_client).await;
    return pokemon_species.unwrap();
}

pub async fn get_pokemon_by_id(id: i64) -> pokemon::Pokemon {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = get_by_id(id, &rustemon_client).await;
    return pokemon.unwrap();
}

async fn get_move_by_name(name: String) -> Move {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let move_ = rustemon::moves::move_::get_by_name(&name, &rustemon_client).await;
    return move_.unwrap();
}

async fn get_machine_by_id(id: i64) -> Machine {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let machine = rustemon::machines::machine::get_by_id(id, &rustemon_client).await;
    return machine.unwrap();
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

pub async fn get_pokemon_moves(
    pokemon: pokemon::Pokemon,
) -> HashMap<String, HashMap<String, HashSet<(String, String)>>> {
    let mut moves_by_version_group: HashMap<String, HashMap<String, HashSet<(String, String)>>> =
        HashMap::new();

    for move_ in &pokemon.moves {
        for detail in &move_.version_group_details {
            let version_group = &detail.version_group.name;

            let version_group_moves = moves_by_version_group
                .entry(version_group.to_string())
                .or_insert_with(|| {
                    let mut methods: HashMap<String, HashSet<(String, String)>> = HashMap::new();
                    methods.insert("level-up".to_string(), HashSet::new());
                    methods.insert("machine".to_string(), HashSet::new());
                    methods.insert("egg".to_string(), HashSet::new());
                    methods.insert("tutor".to_string(), HashSet::new());
                    methods
                });

            let method = &detail.move_learn_method.name;
            if let Some(moves) = version_group_moves.get_mut(method) {
                if method == "level-up" {
                    moves.insert((
                        move_.move_.name.to_string(),
                        detail.level_learned_at.to_string(),
                    ));
                } else if method == "machine" {
                    moves.insert((move_.move_.name.to_string(), "".to_string()));
                } else {
                    moves.insert((move_.move_.name.to_string(), "".to_string()));
                }
            }
        }
    }
    moves_by_version_group
}

pub async fn get_evs(pokemon: pokemon::Pokemon) -> Vec<String> {
    let evs: Vec<String> = pokemon
        .stats
        .iter()
        .map(|stat| stat.effort.to_string())
        .collect();
    return evs;
}

pub fn get_evolution_chain(species: PokemonSpecies) -> ApiResource<EvolutionChain> {
    species.evolution_chain.unwrap()
}

pub async fn get_evolution_chain_id(mut evo_chain: ApiResource<EvolutionChain>) -> i64 {
    let evo_chain_id = evo_chain.url.split_off(42).replace("/", "").parse();
    let evo_chain_id: i64 = evo_chain_id.unwrap();
    return evo_chain_id;
}
