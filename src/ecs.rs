use crate::ecs::components::Velocity;
use bevy_ecs::schedule::ScheduleLabel;
use components::{CoinKind, Position, Rectangle, Solid};
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::ops::Deref;

pub mod components;
pub mod draw_systems;
pub mod input;
pub mod startup_systems;

#[derive(Debug, ScheduleLabel, Clone, Eq, Hash, PartialEq)]
pub struct Update;

#[derive(Debug, Clone, Copy)]
pub struct Entity(usize);

impl Deref for Entity {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Ecs {
    positions: Vec<Option<RefCell<Position>>>,
    velocities: Vec<Option<RefCell<Velocity>>>,
    rects: Vec<Option<Rectangle>>,
    colors: Vec<Option<RefCell<Color>>>,
    coin_kinds: Vec<Option<CoinKind>>,
    solids: Vec<Option<Solid>>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            positions: vec![],
            velocities: vec![],
            rects: vec![],
            colors: vec![],
            coin_kinds: vec![],
            solids: vec![],
        }
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.positions.push(None);
        self.velocities.push(None);
        self.rects.push(None);
        self.colors.push(None);
        self.coin_kinds.push(None);
        self.solids.push(None);
        self
    }

    pub fn with_position(&mut self, position: Position) -> &mut Self {
        self.positions
            .last_mut()
            .and_then(|last| last.replace(RefCell::new(position)));
        self
    }

    pub fn with_velocity(&mut self, velocity: Velocity) -> &mut Self {
        self.velocities
            .last_mut()
            .and_then(|last| last.replace(RefCell::new(velocity)));
        self
    }

    pub fn with_rect(&mut self, rect: Rectangle) -> &mut Self {
        self.rects.last_mut().and_then(|last| last.replace(rect));
        self
    }

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.colors
            .last_mut()
            .and_then(|last| last.replace(RefCell::new(color)));
        self
    }

    pub fn with_solids(&mut self, solid: Solid) -> &mut Self {
        self.solids.last_mut().and_then(|last| last.replace(solid));
        self
    }

    pub fn with_coin_kind(&mut self, coin_kind: CoinKind) -> &mut Self {
        self.colors
            .last_mut()
            .and_then(|last_color| match coin_kind {
                CoinKind::Color(color) => last_color.replace(RefCell::new(color)),
                CoinKind::Jump(_) => last_color.replace(RefCell::new(Color::CYAN)),
            });
        self.coin_kinds
            .last_mut()
            .and_then(|last| last.replace(coin_kind));
        self
    }

    pub fn entity(&self) -> Entity {
        Entity(self.positions.len() - 1)
    }

    pub fn positions(&self) -> &[Option<RefCell<Position>>] {
        &self.positions
    }

    pub fn velocities(&self) -> &[Option<RefCell<Velocity>>] {
        &self.velocities
    }

    pub fn rects(&self) -> &[Option<Rectangle>] {
        &self.rects
    }

    pub fn colors(&self) -> &[Option<RefCell<Color>>] {
        &self.colors
    }

    pub fn coin_kinds(&self) -> &[Option<CoinKind>] {
        &self.coin_kinds
    }

    pub fn solids(&self) -> &[Option<Solid>] {
        &self.solids
    }
}
