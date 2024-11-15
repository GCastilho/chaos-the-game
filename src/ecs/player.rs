use crate::ecs::{
    components::{CoinKind, Color, Player, Position, Rectangle, Velocity},
    physics_systems::ToHitbox,
};
use bevy_ecs::query::Without;
use bevy_ecs::{
    prelude::{Component, Query},
    query::With,
};

pub fn player_collides_coin(
    mut player: Query<(&mut Color, &mut Position, &Rectangle, &mut Velocity), With<Player>>,
    mut coins: Query<(&CoinKind, &mut Position, &Rectangle), Without<Player>>,
) {
    let (mut player_color, mut pos, rect, mut vel) = player.single_mut();
    let player_hitbox = rect.on_position_bevy(pos.reborrow());
    for (kind, mut pos, rect) in coins.iter_mut() {
        let hitbox = rect.on_position_bevy(pos.reborrow());
        if player_hitbox.colides_with(&hitbox) {
            match kind {
                CoinKind::Color(color) => player_color.0 = color.clone(),
                CoinKind::Jump(amount) => vel.y = *amount as i32,
            }
        }
    }
}

#[derive(Debug, Component, Default)]
pub struct Jump {
    frames_to_jump: Option<usize>,
    pub grounded: bool,
}

impl Jump {
    fn max_jump_frames() -> usize {
        30
    }

    fn frames_to_jump(&self) -> usize {
        self.frames_to_jump.unwrap_or(0)
    }

    fn update_velocity(&self, velocity: &mut Velocity) {
        velocity.y = (Jump::max_jump_frames()
            - (Jump::max_jump_frames() - self.frames_to_jump() / 2)) as i32;
    }

    pub fn do_jump(&mut self, vel: &mut Velocity) {
        match (self.grounded, self.frames_to_jump) {
            (true, None) => {
                self.grounded = false;
                self.frames_to_jump = Some(Self::max_jump_frames());
                self.update_velocity(vel);
            }
            (false, Some(v)) if v > 0 => {
                self.update_velocity(vel);
            }
            _ => (),
        }
    }

    pub fn clear_jump(&mut self) {
        self.frames_to_jump = None;
    }
}

pub fn update_jump_time(mut query: Query<&mut Jump>) {
    for mut jump in query.iter_mut() {
        if let Some(v) = &mut jump.frames_to_jump {
            *v = v.saturating_sub(1);
        }
    }
}
