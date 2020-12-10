#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum GameObjectTypes {
    Player,
    Heart,
    Background,
    Feature,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameObjectfeatureTypes {
    Pit1,
    Ladder,
}
