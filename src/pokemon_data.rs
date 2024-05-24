use std::collections::HashMap;
use std::collections::HashSet;

pub struct PokemonData {
    pub id: i64,
    pub name: String,
    pub weight: i64,
    pub height: i64,
    pub generations: Vec<String>,
    pub abilities: Vec<String>,
    pub hidden_ability: String,
    pub types: Vec<String>,
    pub base_stats: Vec<String>,
    pub front_sprite_default: String,
    pub front_sprite_shiny: String,
    pub front_female_sprite_default: String,
    pub front_female_sprite_shiny: String,
    pub has_female_form: bool,
    pub moves: HashMap<String, HashMap<String, HashSet<(String, String)>>>,
    pub evs: Vec<String>, // pub evolution_chain: Vec<(String, String, String)>,
}
