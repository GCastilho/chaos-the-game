pub mod hitbox;

use bevy_ecs::{bundle::Bundle, prelude::Component};
use enum_map::EnumMap;
use hitbox::HitboxBorrowedMut;
use sdl2::pixels::Color;

use crate::game::components::hitbox::HitboxBorrowed;
pub use hitbox::{CollisionAxis, Hitbox};

#[derive(Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Bullet;

#[derive(Debug, Bundle)]
pub struct BulletBundle {
    pub marker: Bullet,
    pub position: Position,
    pub velocity: Velocity,
    pub rectangle: Rectangle,
    pub solid: Solid,
    pub color: Colorable,
    pub bounce: Bounce,
}

#[derive(Debug, Copy, Component, Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    pub fn normalize(mut self) {
        let magnitude = ((self.x).powi(2) + (self.y).powi(2)).sqrt();
        if magnitude == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
        } else {
            self.x = self.x / magnitude;
            self.y = self.y / magnitude;
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Velocity {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Component)]
pub struct Gravitable;

#[derive(Debug, Clone, Copy, Component, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn on_position<'a>(&'a self, position: &'a Position) -> Hitbox<HitboxBorrowed<'a>> {
        let hitbox = HitboxBorrowed {
            rect: self,
            pos: position,
        };
        Hitbox::new(hitbox)
    }

    pub fn on_position_mut<'a>(
        &'a self,
        position: &'a mut Position,
    ) -> Hitbox<HitboxBorrowedMut<'a>> {
        let hitbox = HitboxBorrowedMut {
            rect: self,
            pos: position,
        };
        Hitbox::new(hitbox)
    }
}

pub trait Componentable {
    fn into_fill(self) -> Colorable;
    fn into_outline(self) -> Colorable;
}

impl Componentable for Color {
    fn into_fill(self) -> Colorable {
        Colorable {
            color: self,
            draw_type: ColorDrawType::Fill,
        }
    }

    fn into_outline(self) -> Colorable {
        Colorable {
            color: self,
            draw_type: ColorDrawType::Outline,
        }
    }
}

#[derive(Debug)]
pub enum ColorDrawType {
    Fill,
    Outline,
}

#[derive(Debug, Component)]
pub struct Colorable {
    pub color: Color,
    pub draw_type: ColorDrawType,
}

#[derive(Debug, Component)]
pub enum CoinKind {
    Color(Color),
    Jump(u32),
}

#[derive(Debug, enum_map::Enum)]
pub enum SolidSides {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Solid {
    sides: EnumMap<SolidSides, bool>,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Bounce {
    pub enabled: bool,
    pub bounciness: f64,
}
impl Bounce {
    pub fn new<T: Into<f64>>(enabled: bool, bounciness: T) -> Self {
        Self {
            enabled: enabled,
            bounciness: bounciness.into(),
        }
    }
}

impl Solid {
    pub fn all() -> Self {
        Self {
            sides: EnumMap::from_fn(|_| true),
        }
    }

    pub fn on_any(&self) -> bool {
        self.sides.iter().any(|(_, s)| *s)
    }
}
