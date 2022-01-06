use super::positioning::*;
use std::time::Instant;

pub fn objects_collide(a: &Physics, b: &Physics) -> bool {
    let collision = a.get_position().x - a.get_size().w_half()
        < b.get_position().x + b.get_size().w_half()
        && a.get_position().x + a.get_size().w_half() > b.get_position().x - b.get_size().w_half()
        && a.get_position().y - a.get_size().h_half() < b.get_position().y + b.get_size().h_half()
        && a.get_size().h_half() + a.get_position().y > b.get_position().y - b.get_size().h_half();
    return collision;
}

#[derive(Clone)]
pub struct Interaction {
    pub began_at: Instant,
    pub hovered_option: usize,
    pub options: Option<Vec<String>>,
    pub sub_interactions: Option<Vec<Interaction>>,
    pub dialog: String,
}

impl Interaction {
    pub fn new(
        options: Option<Vec<String>>,
        sub_interactions: Option<Vec<Interaction>>,
        dialog: String,
    ) -> Interaction {
        Interaction {
            began_at: Instant::now(),
            hovered_option: 0,
            options,
            sub_interactions,
            dialog,
        }
    }
}
