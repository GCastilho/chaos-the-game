use components::{CoinKind, Position, Rectangle};
use sdl2::pixels::Color;

pub mod components;

pub struct Ecs {
    positions: Vec<Option<Position>>,
    rects: Vec<Option<Rectangle>>,
    colors: Vec<Option<Color>>,
    coin_kinds: Vec<Option<CoinKind>>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            positions: vec![],
            rects: vec![],
            colors: vec![],
            coin_kinds: vec![],
        }
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.positions.push(None);
        self.rects.push(None);
        self.colors.push(None);
        self.coin_kinds.push(None);
        self
    }

    pub fn with_position(&mut self, position: Position) -> &mut Self {
        self.positions
            .last_mut()
            .and_then(|last| last.replace(position));
        self
    }

    pub fn with_rect(&mut self, rect: Rectangle) -> &mut Self {
        self.rects.last_mut().and_then(|last| last.replace(rect));
        self
    }

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.colors.last_mut().and_then(|last| last.replace(color));
        self
    }

    pub fn with_coind_kind(&mut self, coin_kind: CoinKind) -> &mut Self {
        self.coin_kinds
            .last_mut()
            .and_then(|last| last.replace(coin_kind));
        self.colors
            .last_mut()
            .and_then(|last_color| match coin_kind {
                CoinKind::Color(color) => last_color.replace(color),
                CoinKind::Jump(_) => last_color.replace(Color::CYAN),
            });
        self
    }

    pub fn positions(&self) -> &[Option<Position>] {
        &self.positions
    }

    pub fn rects(&self) -> &[Option<Rectangle>] {
        &self.rects
    }

    pub fn colors(&self) -> &[Option<Color>] {
        &self.colors
    }

    pub fn coin_kinds(&self) -> &[Option<CoinKind>] {
        &self.coin_kinds
    }
}
