use bevy_ecs::schedule::ScheduleLabel;

pub mod components;
pub mod draw_systems;
pub mod input;
pub mod physics_systems;
pub mod player;
pub mod startup_systems;

#[derive(Debug, ScheduleLabel, Clone, Eq, Hash, PartialEq)]
pub struct Update;
