use crate::game::components::{Position, Rectangle};
use sdl2::pixels::Color;
use serde::{Deserialize, Deserializer};
use std::{ops::Deref, str::FromStr};

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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "entity", rename_all = "lowercase")]
pub enum Entity {
    Player {
        position: Position,
        rectangle: Rectangle,
        color: ColorName,
    },
    Static {
        position: Position,
        rectangle: Rectangle,
        color: ColorName,
    },
    Coin {
        position: Position,
        rectangle: Rectangle,
        color: ColorName,
        jump: Option<f64>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP_JSON: &str = "[{\"entity\":\"player\",\"position\":{\"x\":250,\"y\":600},\"rectangle\":{\"width\":50,\"height\":50},\"color\":\"blue\"},{\"entity\":\"static\",\"position\":{\"x\":250,\"y\":600},\"rectangle\":{\"width\":50,\"height\":50},\"color\":\"green\"},{\"entity\":\"coin\",\"position\":{\"x\":120,\"y\":115},\"rectangle\":{\"width\":10,\"height\":10},\"color\":\"magenta\"},{\"entity\":\"coin\",\"position\":{\"x\":470,\"y\":115},\"rectangle\":{\"width\":10,\"height\":10},\"color\":\"red\"},{\"entity\":\"coin\",\"position\":{\"x\":300,\"y\":115},\"rectangle\":{\"width\":10,\"height\":10},\"color\":\"cyan\",\"jump\":3}]";

    #[test]
    fn test_parse_map() {
        let expected = vec![
            Entity::Player {
                position: Position { x: 250.0, y: 600.0 },
                rectangle: Rectangle {
                    width: 50,
                    height: 50,
                },
                color: ColorName(Color::BLUE),
            },
            Entity::Static {
                position: Position { x: 250.0, y: 600.0 },
                rectangle: Rectangle {
                    width: 50,
                    height: 50,
                },
                color: ColorName(Color::GREEN),
            },
            Entity::Coin {
                position: Position { x: 120.0, y: 115.0 },
                rectangle: Rectangle {
                    width: 10,
                    height: 10,
                },
                color: ColorName(Color::MAGENTA),
                jump: None,
            },
            Entity::Coin {
                position: Position { x: 470.0, y: 115.0 },
                rectangle: Rectangle {
                    width: 10,
                    height: 10,
                },
                color: ColorName(Color::RED),
                jump: None,
            },
            Entity::Coin {
                position: Position { x: 300.0, y: 115.0 },
                rectangle: Rectangle {
                    width: 10,
                    height: 10,
                },
                color: ColorName(Color::CYAN),
                jump: Some(3.0),
            },
        ];
        let entities = serde_json::from_str::<Vec<Entity>>(MAP_JSON).expect("parse failed");
        assert_eq!(expected, entities);
    }
}
