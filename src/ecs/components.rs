use enum_map::EnumMap;
use sdl2::pixels::Color;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn on_position<'a>(&'a self, position: &'a Position) -> Hitbox<'a> {
        Hitbox {
            pos: position,
            rect: self,
        }
    }
}

#[derive(Debug)]
pub struct Hitbox<'a> {
    pos: &'a Position,
    rect: &'a Rectangle,
}

impl<'a> Hitbox<'a> {
    pub fn left(&self) -> i32 {
        self.pos.x
    }

    pub fn right(&self) -> i32 {
        self.pos.x + self.rect.width as i32
    }

    pub fn top(&self) -> i32 {
        self.pos.y + self.rect.height as i32
    }

    pub fn bottom(&self) -> i32 {
        self.pos.y
    }

    pub fn colides_with(&self, other: &'a Hitbox<'a>) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
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
}
