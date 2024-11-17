use bevy_ecs::system::Resource;
use std::time::{Duration, Instant};

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
        self.delta = now - self.last_update;
        self.elapsed += self.delta;
        self.last_update = now;
    }
}
