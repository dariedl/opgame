mod bob;
use crate::data::Character;

pub fn get_character(id: &str) -> Character {
    return match id {
        "bob" => bob::get_bob(),
        _ => panic!("crash and burn"),
    };
}
