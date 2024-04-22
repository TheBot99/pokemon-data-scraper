use tokio;
use rustemon::model::pokemon;
use rustemon::pokemon::pokemon::get_by_id;




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
    let generations: std::collections::HashSet<String> = pokemon.moves.iter()
        .flat_map(|mv| mv.version_group_details.iter())
        .map(|detail| detail.version_group.name.split("-").next().unwrap().to_string())
        .collect();
    let generations_list: Vec<String> = generations.into_iter().map(|s| s.to_string()).collect();
    return generations_list;
}

async fn get_abilities(pokemon: pokemon::Pokemon) -> Vec<String> {
    let abilities: Vec<String> = pokemon.abilities.iter()
        .filter(|abiliti| !abiliti.is_hidden)
        .map(|ability| ability.ability.name.to_string())
        .collect();
    return abilities;
}

async fn get_hidden_abilities(pokemon: pokemon::Pokemon) -> Vec<String> {
    let hidden_abilities: Vec<String> = pokemon.abilities.iter()
        .filter(|ability| ability.is_hidden)
        .map(|ability| ability.ability.name.to_string())
        .collect();
    return hidden_abilities;
}

fn get_data(id: i64) ->  (String, i64, i64, Vec<String>, Vec<String>, Vec<String>){
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    let pokemon = runtime.block_on(async {
        get_pokemon_by_id(id).await
    });

    let pokemon_name = runtime.block_on(async {
        get_pokemon_name(pokemon.clone()).await
    });

    let pokemon_weight = runtime.block_on(async {
        get_pokemon_weight(pokemon.clone()).await
    });

    let pokemon_height = runtime.block_on(async {
        get_pokemon_height(pokemon.clone()).await
    });

    let pokemon_generations = runtime.block_on(async {
        get_pokemon_generations(pokemon.clone()).await
    });

    let pokemon_abilities = runtime.block_on(async {
        get_abilities(pokemon.clone()).await
    });

    let pokemon_hidden_abilities = runtime.block_on(async {
        get_hidden_abilities(pokemon.clone()).await
    });

    return (pokemon_name, pokemon_weight, pokemon_height, pokemon_generations, pokemon_abilities, pokemon_hidden_abilities); 
}

fn main() {
    let mut id:i64   = 768;

    while id <= 768 {
        let (pokemon_name, pokemon_weight, pokemon_height, pokemon_generations, pokemon_abilities, pokemon_hidden_abilities) = get_data(id);
        println!("The name of the id:{} is: {}",id ,pokemon_name);
        println!("");
        println!("The weight of {} is: {}", pokemon_name, pokemon_weight);
        println!("");
        println!("The height of {} is: {}", pokemon_name, pokemon_height);
        println!("");
        println!("The generations of {} are: {:?}", pokemon_name, pokemon_generations);
        println!("");
        println!("The abilities of {} are: {:?}", pokemon_name, pokemon_abilities);
        println!("");
        println!("The hidden abilities of {} are: {:?}", pokemon_name, pokemon_hidden_abilities);
        println!("");
        println!("------------------------------------------------------------");
        id += 1;
    }

}