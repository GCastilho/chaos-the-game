use super::{
    components::{
        CoinKind, Componentable, Gravitable, Player, Position, Rectangle, Solid, Velocity,
    },
    physics::PLAYER_VERTICAL_ACCELERATION,
    player::Jump,
};
use crate::game::components::{Direction, InfiniteArea, KillZone};
use bevy_ecs::prelude::Commands;
use sdl2::pixels::Color;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct ColorName(Color);

impl From<ColorName> for Color {
    fn from(value: ColorName) -> Self {
        value.0
    }
}
impl FromStr for ColorName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = match s {
            "white" => Color::WHITE,
            "black" => Color::BLACK,
            "gray" => Color::GRAY,
            "grey" => Color::GREY,
            "red" => Color::RED,
            "green" => Color::GREEN,
            "blue" => Color::BLUE,
            "magenta" => Color::MAGENTA,
            "yellow" => Color::YELLOW,
            "cyan" => Color::CYAN,
            _ => return Err(format!("{} is not a valid color name", s)),
        };
        Ok(ColorName(color))
    }
}

impl<'de> Deserialize<'de> for ColorName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KillZoneType {
    Area {
        position: Position,
        rectangle: Rectangle,
    },
    Infinite {
        start: f64,
        direction: Direction,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "entity", rename_all = "snake_case")]
pub enum Entity {
    Player {
        position: Position,
    },
    Static {
        position: Position,
        rectangle: Rectangle,
        color: ColorName,
    },
    Coin {
        position: Position,
        color: ColorName,
        jump: Option<f64>,
    },
    KillZone(KillZoneType),
}

impl Entity {
    pub fn spawn(self, commands: &mut Commands) {
        match self {
            Entity::Player { position } => {
                commands.spawn((
                    Player,
                    position,
                    Rectangle::new(50, 50),
                    Color::BLUE.into_fill(),
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
                color,
                jump,
            } => {
                let coin_kind = jump
                    .map(|v| CoinKind::Jump((PLAYER_VERTICAL_ACCELERATION / v) as u32))
                    .unwrap_or(CoinKind::Color(color.clone().into()));
                commands.spawn((
                    position,
                    Rectangle::new(10, 10),
                    Color::from(color).into_fill(),
                    coin_kind,
                ));
            }
            Entity::KillZone(zone_type) => match zone_type {
                KillZoneType::Area {
                    position,
                    rectangle,
                } => {
                    commands.spawn((
                        KillZone,
                        position,
                        rectangle,
                        Color::RGBA(255, 0, 0, 64).into_fill(),
                    ));
                }
                KillZoneType::Infinite { start, direction } => {
                    let infinite_area = InfiniteArea { start, direction };
                    commands.spawn((KillZone, infinite_area));
                }
            },
        }
    }
}
