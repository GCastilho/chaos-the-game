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

#[derive(Debug, Clone, Copy)]
pub enum CoinKind {
    Color(Color),
    Jump(u32),
}
