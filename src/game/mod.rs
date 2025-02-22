use bevy_ecs::schedule::ScheduleLabel;

pub mod camera;
pub mod components;
pub mod draw;
pub mod input;
mod map;
pub mod physics;
pub mod player;
pub mod resources;
pub mod startup;

#[derive(Debug, ScheduleLabel, Clone, Eq, Hash, PartialEq)]
pub struct Update;
