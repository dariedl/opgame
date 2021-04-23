use rand::Rng;

use crate::data::Character;
use crate::data::Probe;
// use crate::domainMessage::Command;
use crate::domain_message::Command;
use crate::domain_message::Event;

pub fn handle_state(command: Command) {
    let events = match command {
        Command::DoAdventureProbe(character, probe) => do_adventure_probe(character, probe),
    };
    println!("{:?}", events)
}

fn roll_dice() -> i8 {
    return rand::thread_rng().gen_range(0, 8) - 4;
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
        r if r >= diff => vec![Event::SucceededProbe(probe)],
        _ => vec![Event::SucceededProbe(probe), Event::GainedBerry(2)],
    };
}
