use enum_map::EnumMap;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::cmp::Ordering::*;

// Os componentes não podem ter clone ou copy, ou um update irá atualizar a cópia, não a referência

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Velocity {
    pub gravitable: bool,
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn new_gravitable(x: i32, y: i32) -> Self {
        Self {
            gravitable: true,
            x,
            y,
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new_gravitable(0, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn on_position<'a>(&'a self, position: &'a RefCell<Position>) -> Hitbox<'a> {
        Hitbox {
            pos: position,
            rect: self,
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

#[derive(Debug)]
pub struct Hitbox<'a> {
    pub pos: &'a RefCell<Position>,
    pub rect: &'a Rectangle,
}

impl<'a> Hitbox<'a> {
    pub fn left(&self) -> i32 {
        self.pos.borrow().x
    }

    pub fn right(&self) -> i32 {
        self.pos.borrow().x + self.rect.width as i32
    }

    pub fn top(&self) -> i32 {
        self.pos.borrow().y + self.rect.height as i32
    }

    pub fn bottom(&self) -> i32 {
        self.pos.borrow().y
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

        let (y_axis, y_value) = match y_up.cmp(&y_down) {
            Greater | Equal => (CollisionAxis::Down, y_down),
            Less => (CollisionAxis::Up, y_up),
        };

        let (x_axis, x_value) = match x_left.cmp(&x_right) {
            Greater | Equal => (CollisionAxis::Right, x_right),
            Less => (CollisionAxis::Left, x_left),
        };

        match y_value.cmp(&x_value) {
            Greater => Some(x_axis),
            Less => Some(y_axis),
            Equal => None,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
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
