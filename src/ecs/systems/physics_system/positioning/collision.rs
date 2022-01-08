use super::positioning::*;

pub fn objects_collide(a: &Physics, b: &Physics) -> bool {
    let collision = a.get_position().x - a.get_size().w_half()
        < b.get_position().x + b.get_size().w_half()
        && a.get_position().x + a.get_size().w_half() > b.get_position().x - b.get_size().w_half()
        && a.get_position().y - a.get_size().h_half() < b.get_position().y + b.get_size().h_half()
        && a.get_size().h_half() + a.get_position().y > b.get_position().y - b.get_size().h_half();
    return collision;
}

#[derive(Clone, Deserialize, Debug)]
pub struct Interaction {
    #[serde(default)]
    pub hovered_option: usize,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub sub_interactions: Option<Vec<Interaction>>,
    #[serde(default)]
    pub dialog: String,
}

impl Interaction {
    pub fn new(
        options: Option<Vec<String>>,
        sub_interactions: Option<Vec<Interaction>>,
        dialog: String,
    ) -> Interaction {
        Interaction {
            hovered_option: 0,
            options,
            sub_interactions,
            dialog,
        }
    }
}

impl Default for Interaction {
    fn default() -> Interaction {
        Interaction {
            hovered_option: 0,
            options: None,
            sub_interactions: None,
            dialog: "Hi!".to_string(),
        }
    }
}
