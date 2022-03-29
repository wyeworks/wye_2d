use std::fs::read_to_string;
use std::path::Path;

use super::super::systems::physics_system::positioning::collision::Interaction;

#[derive(Deserialize, Debug)]
pub struct NpcJson {
    pub id: u32,
    pub name: String,
    pub main_interaction: Option<Interaction>,
}
#[derive(Deserialize, Debug)]
struct JsonObject {
    npcs: Vec<NpcJson>,
}

pub fn load_npcs() -> Vec<NpcJson> {
    let json_file_path = Path::new("src/resources/npcs.json");
    let json_file_str = read_to_string(json_file_path).expect("file not found");

    let deserialized_object: JsonObject =
        serde_json::from_str::<JsonObject>(&json_file_str).expect("error while reading json");
    deserialized_object.npcs
}
