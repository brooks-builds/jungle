pub mod player_life_system;

use std::fmt::Debug;

pub trait LifeSystem
where
    Self: Debug,
{
    fn get_lives(&self) -> u8;
}
