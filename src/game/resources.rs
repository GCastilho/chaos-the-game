use crate::game::components::Position;
use bevy_ecs::system::Resource;
use std::{
    cmp,
    ops::Deref,
    time::{Duration, Instant},
};

#[derive(Debug, Resource)]
pub struct Time {
    last_update: Instant,
    delta: Duration,
    elapsed: Duration,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
        }
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = cmp::min(now - self.last_update, Self::MAX_DELTA);
        self.elapsed += self.delta;
        self.last_update = now;
    }

    // TODO: Talvez o máximo deveria ser 100. Coloquei 50 pq com 100 ainda atravessava o chão
    const MAX_DELTA: Duration = Duration::from_millis(50);
}

#[derive(Debug, Resource, Clone)]
pub struct Spawn(pub Position);

impl Deref for Spawn {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Spawn {
    pub fn new() -> Self {
        Self(Position::new(0, 0))
    }
}
