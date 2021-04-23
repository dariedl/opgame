use crate::data::Character;
use crate::data::Probe;

#[derive(Debug)]
pub enum Command {
    DoAdventureProbe(Character, Probe),
}

#[derive(Debug)]
pub enum Event {
    // DiceRolled(i8),
    SucceededProbe(Probe), //character + diceroll
    FailedProbe(Probe),
    GainedBerry(u8),
    InjuredCharacter(Character),
    // CharacterTapped(Character)
}

pub fn printCmd(cmd: &Command) {
    match cmd {
        Command::DoAdventureProbe(character, probe) => println!(
            "You character {} attemtps an adventureprobe of difficulty {}",
            character.name, probe.difficulty
        ),
    }
}

pub fn printEvent(event: Event) {
    match event {
        Event::SucceededProbe(_) => println!("You succeeded your probe!!! You're awesome!"),
        Event::FailedProbe(_) => println!("YOU LOSE!!"),
        Event::GainedBerry(berry) => println!("You gained {} berry", berry),
        Event::InjuredCharacter(c) => println!("Your character {} has injured himself", c.name),
    }
}
// Action: {Player, Command, Event[]}
