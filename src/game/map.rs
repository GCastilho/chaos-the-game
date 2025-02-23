use crate::game::components::{Position, Rectangle};
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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "entity", rename_all = "lowercase")]
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
}
