use bevy_ecs::{bundle::Bundle, prelude::Component};
use enum_map::EnumMap;
use std::cmp::Ordering::*;
use std::ops::Deref;

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
    pub color: Color,
}

#[derive(Debug, Component, Clone)]
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

#[derive(Debug, Clone, Copy, Component)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn on_position<'a>(&'a self, position: &'a mut Position) -> Hitbox<'a> {
        Hitbox {
            rect: self,
            pos: position,
        }
    }
}

#[derive(Debug)]
pub enum CollisionAxis {
    Up,
    Down,
    Left,
    Right,
}

// TODO: Seria interessante se tivesse como post ser mutável ou não dependendo de como ela é inicializada
pub struct Hitbox<'a> {
    pub pos: &'a mut Position,
    pub rect: &'a Rectangle,
}

impl<'a> Hitbox<'a> {
    pub fn left(&self) -> f64 {
        self.pos.x
    }

    pub fn right(&self) -> f64 {
        self.pos.x + self.rect.width as f64
    }

    pub fn top(&self) -> f64 {
        self.pos.y + self.rect.height as f64
    }

    pub fn bottom(&self) -> f64 {
        self.pos.y
    }

    pub fn colides_with(&self, other: &'a Hitbox<'a>) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.bottom() < other.top()
            && self.top() > other.bottom()
    }

    pub fn colides_with_axis(&self, other: &Hitbox) -> Option<CollisionAxis> {
        if !self.colides_with(other) {
            return None;
        }

        let y_up = self.top() - other.bottom();
        let y_down = other.top() - self.bottom();
        let x_right = self.right() - other.left();
        let x_left = other.right() - self.left();

        let (y_axis, y_value) = match y_up.total_cmp(&y_down) {
            Greater | Equal => (CollisionAxis::Down, y_down),
            Less => (CollisionAxis::Up, y_up),
        };

        let (x_axis, x_value) = match x_left.total_cmp(&x_right) {
            Greater | Equal => (CollisionAxis::Right, x_right),
            Less => (CollisionAxis::Left, x_left),
        };

        match y_value.total_cmp(&x_value) {
            Greater => Some(x_axis),
            Less => Some(y_axis),
            Equal => None,
        }
    }
}

pub trait Componentable {
    fn into_component(self) -> Color;
}

impl Componentable for sdl2::pixels::Color {
    fn into_component(self) -> Color {
        Color(self)
    }
}

#[derive(Debug, Component)]
pub struct Color(pub sdl2::pixels::Color);

impl Deref for Color {
    type Target = sdl2::pixels::Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Component)]
pub enum CoinKind {
    Color(sdl2::pixels::Color),
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
