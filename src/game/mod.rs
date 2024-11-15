use bevy_ecs::schedule::ScheduleLabel;

pub mod components;
pub mod draw;
pub mod input;
pub mod physics;
pub mod player;
pub mod startup;

#[derive(Debug, ScheduleLabel, Clone, Eq, Hash, PartialEq)]
pub struct Update;
