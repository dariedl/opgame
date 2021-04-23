use rand::Rng;

use crate::data::Character;
use crate::data::Probe;
// use crate::domainMessage::Command;
use crate::domain_message::printCmd;
use crate::domain_message::printEvent;
use crate::domain_message::Command;
use crate::domain_message::Event;

pub fn handle_state(command: Command) {
    printCmd(&command);
    let events = match command {
        Command::DoAdventureProbe(character, probe) => do_adventure_probe(character, probe),
    };
    for event in events {
        printEvent(event)
    }
}

fn roll_dice() -> i8 {
    let dice_roll = rand::thread_rng().gen_range(0, 8) - 4;
    println!("Diceroll: {}!", dice_roll);
    return dice_roll;
}

pub fn do_adventure_probe(character: Character, probe: Probe) -> Vec<Event> {
    let attribute = character.fight as i8;
    let diff = probe.difficulty as i8;
    let result = roll_dice() + attribute;
    return match result {
        r if r < diff - 1 => vec![
            Event::FailedProbe(probe),
            Event::InjuredCharacter(character),
        ],
        r if r < diff => vec![Event::FailedProbe(probe)],
        r if r >= diff + 2 => vec![Event::SucceededProbe(probe), Event::GainedBerry(2)],
        _ => vec![Event::SucceededProbe(probe)],
    };
}
