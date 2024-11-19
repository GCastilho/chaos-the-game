use super::components::{
    Bullet, BulletBundle, CoinKind, Color, Componentable, Player, Position, Rectangle, Solid,
    Velocity,
};
use crate::game::{
    input::{Action, InputEvent, InputState},
    physics::PLAYER_MAX_VERTICAL_SPEED,
};
use bevy_ecs::change_detection::Res;
use bevy_ecs::event::EventReader;
use bevy_ecs::query::Without;
use bevy_ecs::system::Commands;
use bevy_ecs::{
    prelude::{Component, Query},
    query::With,
};
use std::cmp::Ordering::{Equal, Greater, Less};

const PLAYER_MAX_HORIZONTAL_SPEED: f64 = 15.0;
const PLAYER_HORIZONTAL_ACCELERATION: f64 = 1.0;
const JUMP_FRAMES: usize = 30;

pub fn handle_player_input(
    mut query: Query<(&mut Velocity, &mut Jump), With<Player>>,
    mut ev_input: EventReader<InputEvent>,
    inputs: Res<InputState>,
    mut player_position: Query<&Position, With<Player>>,
    mut commands: Commands,
) {
    for (mut velocity, mut jump) in query.iter_mut() {
        if inputs.state()[Action::Left].active() && velocity.x >= -PLAYER_MAX_HORIZONTAL_SPEED {
            velocity.x -= PLAYER_HORIZONTAL_ACCELERATION;
        }
        if inputs.state()[Action::Right].active() && velocity.x <= PLAYER_MAX_HORIZONTAL_SPEED {
            velocity.x += PLAYER_HORIZONTAL_ACCELERATION;
        }

        if !inputs.state()[Action::Left].active() && !inputs.state()[Action::Right].active() {
            match velocity.x.total_cmp(&0.0) {
                Less => velocity.x += PLAYER_HORIZONTAL_ACCELERATION,
                Greater => velocity.x -= PLAYER_HORIZONTAL_ACCELERATION,
                Equal => (),
            }
        }

        if inputs.state()[Action::Up].active() && velocity.y <= PLAYER_MAX_VERTICAL_SPEED {
            jump.do_jump(&mut velocity);
        } else {
            jump.clear_jump()
        }

        if inputs.state()[Action::Down].active() && velocity.y >= -PLAYER_MAX_VERTICAL_SPEED {
            velocity.y -= 10.0;
        }
    }

    let player_position = player_position.single();

    for ev in ev_input.read() {
        if ev.state.active() && ev.action == Action::Atack {
            commands.spawn(BulletBundle {
                marker: Bullet,
                position: Position::new(player_position.x + 60.0, player_position.y + 25.0),
                velocity: Velocity::new(10, 0),
                rectangle: Rectangle::new(10, 10),
                solid: Solid::all(),
                color: sdl2::pixels::Color::RED.into_component(),
            });
        };
    }
}

pub fn player_collides_coin(
    mut player: Query<(&mut Color, &mut Position, &Rectangle, &mut Velocity), With<Player>>,
    mut coins: Query<(&CoinKind, &mut Position, &Rectangle), Without<Player>>,
) {
    let (mut player_color, mut pos, rect, mut vel) = player.single_mut();
    let player_hitbox = rect.on_position(&mut *pos);
    for (kind, mut pos, rect) in coins.iter_mut() {
        let hitbox = rect.on_position(&mut *pos);
        if player_hitbox.colides_with(&hitbox) {
            match kind {
                CoinKind::Color(color) => player_color.0 = color.clone(),
                CoinKind::Jump(amount) => vel.y = *amount as f64,
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
    fn frames_to_jump(&self) -> usize {
        self.frames_to_jump.unwrap_or(0)
    }

    fn update_velocity(&self, velocity: &mut Velocity) {
        velocity.y = (JUMP_FRAMES - (JUMP_FRAMES - self.frames_to_jump() / 2)) as f64;
    }

    pub fn do_jump(&mut self, vel: &mut Velocity) {
        match (self.grounded, self.frames_to_jump) {
            (true, None) => {
                self.grounded = false;
                self.frames_to_jump = Some(JUMP_FRAMES);
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
