use super::{
    components::{CoinKind, Componentable, Gravitable, Player, Solid, Velocity},
    map::Entity,
    physics::PLAYER_VERTICAL_ACCELERATION,
    player::Jump,
};
use bevy_ecs::{schedule::ScheduleLabel, system::Commands};
use sdl2::pixels::Color;
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Startup;

pub fn init_map_system(mut commands: Commands) {
    let map_file = fs::read_to_string("assets/maps/map_01.json").expect("Map file not found");
    let entities = serde_json::from_str::<Vec<Entity>>(&map_file).expect("Failed to parse map");

    let mut found_player = false;
    for entity in entities {
        match entity {
            Entity::Player {
                position,
                rectangle,
                color,
            } => {
                if found_player {
                    panic!("Map defined more than one player");
                }
                found_player = true;
                commands.spawn((
                    Player,
                    position,
                    rectangle,
                    Color::from(color).into_fill(),
                    Velocity::default(),
                    Solid::all(),
                    Jump::default(),
                    Gravitable,
                ));
            }
            Entity::Static {
                position,
                rectangle,
                color,
            } => {
                commands.spawn((
                    position,
                    rectangle,
                    Color::from(color).into_fill(),
                    Solid::all(),
                ));
            }
            Entity::Coin {
                position,
                rectangle,
                color,
                jump,
            } => {
                let coin_kind = jump
                    .map(|v| CoinKind::Jump((PLAYER_VERTICAL_ACCELERATION / v) as u32))
                    .unwrap_or(CoinKind::Color(color.clone().into()));
                commands.spawn((
                    position,
                    rectangle,
                    Color::from(color).into_fill(),
                    coin_kind,
                ));
            }
        }
    }
}
