use crate::data::Character;

pub fn get_bob() -> Character {
    return Character {
        id: String::from("bob"),
        name: String::from("Bob"),
        berry: 2,
        leadership: 3,
        fight: 2,
        intel: 1,
        tech: 5,
        support: 4,
    };
}
