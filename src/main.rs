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



fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    let pokemon = runtime.block_on(async {
        get_pokemon_by_id(1).await
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

    println!("The name of the Pokemon is: {}", pokemon_name);
    println!("The weight of {} is: {}", pokemon_name, pokemon_weight);
    println!("The height of {} is: {}", pokemon_name, pokemon_height);
    println!("The generations of {} are: {:?}", pokemon_name, pokemon_generations);
}