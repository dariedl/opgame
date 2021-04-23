#[derive(Debug)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub berry: u64,
    pub leadership: u64,
    pub fight: u64,
    pub intel: u64,
    pub tech: u64,
    pub support: u64,
}

#[derive(Debug)]
pub struct Probe {
    pub id: String,
    pub difficulty: u64,
    pub parent_id: u64,
}
