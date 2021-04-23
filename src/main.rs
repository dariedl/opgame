use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let character = &args[1];
    let action = &args[2];

    println!("Character {} ", character);
    println!("sails out to adventure {}", action);

    let Bob = Character {
        id: 1,
        // name: "Bob",
        berry: 2,
        leadership: 3,
        fight: 2,
        intel: 1,
        tech: 5,
        support: 4,
    };

    let firstIsland = Probe {
        id: 1,
        difficulty: 1,
    };
    handleState(Bob, firstIsland)
}
#[derive(Debug)]
struct Character {
    id: u64,
    // name: string,
    berry: u64,
    leadership: u64,
    fight: u64,
    intel: u64,
    tech: u64,
    support: u64,
}
#[derive(Debug)]
struct Probe {
    id: u64,
    difficulty: u64,
}
#[derive(Debug)]
enum Command {}

#[derive(Debug)]
enum Event {
    SucceededProbe(Probe),
    FailedProbe(Probe),
    GainedBerry(u8),
    InjuredCharacter(Character),
}
fn handleState(character: Character, probe: Probe) {
    let events = doAdventureProbe(character, probe);
    println!("{:?}", events)
}

fn rollDice() -> i32 {
    return rand::thread_rng().gen_range(0, 8) - 4;
}
fn doAdventureProbe(character: Character, probe: Probe) -> Vec<Event> {
    let attribute = character.fight as i32;
    let diff = probe.difficulty as i32;
    let result = rollDice() + attribute;
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
