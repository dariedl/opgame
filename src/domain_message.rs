use crate::data::Character;
use crate::data::Probe;

#[derive(Debug)]
pub enum Command {
    DoAdventureProbe(Character, Probe),
}

#[derive(Debug)]
pub enum Event {
    SucceededProbe(Probe),
    FailedProbe(Probe),
    GainedBerry(u8),
    InjuredCharacter(Character),
}
