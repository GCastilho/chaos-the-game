use super::map::Entity;
use crate::game::resources::Spawn;
use bevy_ecs::{
    schedule::ScheduleLabel,
    system::{Commands, ResMut},
};
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Startup;

pub fn init_map_system(mut commands: Commands, mut spawn: ResMut<Spawn>) {
    let map_file = fs::read_to_string("assets/maps/map_01.json").expect("Map file not found");
    let entities = serde_json::from_str::<Vec<Entity>>(&map_file).expect("Failed to parse map");
    match entities
        .iter()
        .fold(0, |acc, e| acc + matches!(e, Entity::Player { .. }) as i32)
    {
        0 => panic!("Map defined no player"),
        2.. => panic!("Map defined more than one player"),
        _ => (),
    }

    if let Some(Entity::Player { position }) =
        entities.iter().find(|e| matches!(e, Entity::Player { .. }))
    {
        spawn.0 = position.clone();
    }

    for entity in entities {
        entity.spawn(&mut commands);
    }
}
