use super::positioning::*;

pub fn objects_collide(a: &Body, b: &Body) -> bool {
    let collision = a.get_position().x - a.get_size().w_half() < b.get_position().x + b.get_size().w_half()
        && a.get_position().x + a.get_size().w_half() > b.get_position().x - b.get_size().w_half()
        && a.get_position().y - a.get_size().h_half() < b.get_position().y + b.get_size().h_half()
        && a.get_size().h_half() + a.get_position().y > b.get_position().y - b.get_size().h_half();
    return collision;
}