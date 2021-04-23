mod character_data;
mod data;
mod domain_message;
mod game_state;
use crate::character_data::get_character;
use crate::data::Probe;
use crate::domain_message::Command;
use crate::game_state::handle_state;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let character = &args[1];
    // let action = &args[2];

    // println!("Character {} ", character);
    // println!("sails out to adventure {}", action);

    let first_probe = Probe {
        id: String::from("probe"),
        difficulty: 1,
        parent_id: 1,
    };
    handle_state(Command::DoAdventureProbe(
        get_character(character),
        first_probe,
    ))
}
