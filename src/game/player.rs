use super::{
    components::{
        Bounce, Bullet, BulletBundle, CoinKind, Colorable, Componentable, Player, Position,
        Rectangle, Solid, Velocity,
    },
    input::{Action, InputEvent, InputState},
    physics::{
        PLAYER_HORIZONTAL_ACCELERATION, PLAYER_MAX_HORIZONTAL_SPEED, PLAYER_MAX_VERTICAL_SPEED,
        PLAYER_VERTICAL_ACCELERATION,
    },
    resources::Time,
};
use bevy_ecs::{
    change_detection::Res,
    entity::Entity,
    event::EventReader,
    prelude::{Component, Query},
    query::{With, Without},
    system::Commands,
};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    time::Duration,
};

const JUMP_MILLIS: u64 = 500;

pub fn handle_player_input(
    mut query: Query<(&mut Velocity, &mut Jump), With<Player>>,
    inputs: Res<InputState>,
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
}

pub fn player_attack(
    player_position: Query<&Position, With<Player>>,
    player_rec: Query<&Rectangle, With<Player>>,
    mut player_velocity: Query<&mut Velocity, With<Player>>,
    mut commands: Commands,
    mut ev_input: EventReader<InputEvent>,
    query2: Query<(Entity, &Position, &Rectangle), With<Bullet>>,
) {
    let player_position = player_position.single();
    let mut player_velocity = player_velocity.single_mut();
    let player_rec = player_rec.single();
    for ev in ev_input.read() {
        if ev.state.active() && ev.action == Action::Attack {
            commands.spawn(BulletBundle {
                marker: Bullet,
                position: Position::new(player_position.x + 60.0, player_position.y + 25.0),
                velocity: Velocity::new(60, 0),
                rectangle: Rectangle::new(10, 10),
                solid: Solid::all(),
                color: sdl2::pixels::Color::RED.into_fill(),
                bounce: Bounce::new(true, 1.0),
            });

            if let Some(bullet_entity) = query2.iter().next() {
                // let mut b_pos = bullet_entity.1;
                let b_rec = bullet_entity.2;
                let b_pos = &Position::new(
                    bullet_entity.1.x + b_rec.width as f64 / 2.0,
                    bullet_entity.1.y + b_rec.width as f64 / 2.0,
                );
                let p_pos = Position::new(
                    player_position.x + player_rec.width as f64 / 2.0,
                    player_position.y + player_rec.height as f64 / 2.0,
                );
                let distance = ((p_pos.x - b_pos.x).powi(2) + (p_pos.y - b_pos.y).powi(2)).sqrt();

                let vector = Position::new(p_pos.x - b_pos.x, p_pos.y - b_pos.y);
                vector.normalize();
                if distance < 200.0 {
                    player_velocity.x = vector.x * (2.0 as f64).powf(20.0 - distance / 10.0);
                    player_velocity.y = vector.y * (2.0 as f64).powf(20.0 - distance / 10.0);
                }
                commands.entity(bullet_entity.0).despawn(); // Despawn the first matching entity

                println!("Bullet exploded !");
            } else {
                println!("No bullet entity found to explode.");
            }
        };
    }
}

pub fn player_collides_coin(
    mut player: Query<(&mut Colorable, &Position, &Rectangle, &mut Velocity), With<Player>>,
    mut coins: Query<(&CoinKind, &Position, &Rectangle), Without<Player>>,
) {
    let (mut player_color, pos, rect, mut vel) = player.single_mut();
    let player_hitbox = rect.on_position(pos);
    for (kind, pos, rect) in coins.iter_mut() {
        let hitbox = rect.on_position(pos);
        if player_hitbox.colides_with(&hitbox) {
            match kind {
                CoinKind::Color(color) => player_color.color = *color,
                CoinKind::Jump(amount) => vel.y = *amount as f64,
            }
        }
    }
}

#[derive(Debug, Component, Default)]
pub struct Jump {
    time_to_jump: Option<Duration>,
    pub grounded: bool,
}

impl Jump {
    fn update_velocity(&self, velocity: &mut Velocity) {
        if let Some(time) = self.time_to_jump {
            let max_jump_time = Duration::from_millis(JUMP_MILLIS);
            velocity.y = (max_jump_time - (max_jump_time - time / 2)).as_secs_f64()
                * PLAYER_VERTICAL_ACCELERATION;
        }
    }

    pub fn do_jump(&mut self, vel: &mut Velocity) {
        match (self.grounded, self.time_to_jump) {
            (true, None) => {
                self.grounded = false;
                self.time_to_jump = Some(Duration::from_millis(JUMP_MILLIS));
                self.update_velocity(vel);
            }
            (false, Some(v)) if v > Duration::ZERO => {
                self.update_velocity(vel);
            }
            _ => (),
        }
    }

    pub fn clear_jump(&mut self) {
        self.time_to_jump = None;
    }
}

pub fn update_jump_time(mut query: Query<&mut Jump>, time: Res<Time>) {
    for mut jump in query.iter_mut() {
        if let Some(v) = &mut jump.time_to_jump {
            *v = v.saturating_sub(time.delta());
        }
    }
}
