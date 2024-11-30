use super::{
    components::{Bounce, CollisionAxis, Gravitable, Position, Rectangle, Solid, Velocity},
    player::Jump,
    resources::Time,
};
use bevy_ecs::{
    prelude::{Query, Res},
    query::{With, Without},
};

pub const PLAYER_MAX_VERTICAL_SPEED: f64 = 1500.0;
pub const PLAYER_MAX_HORIZONTAL_SPEED: f64 = 900.0;
pub const PLAYER_VERTICAL_ACCELERATION: f64 = 5000.0;
pub const PLAYER_HORIZONTAL_ACCELERATION: f64 = 60.0;

/// Colisão entre coisas com e sem velocidade.
///
/// Não dá para fazer todas as colisões aqui porque elas dão overlap, e isso deixa o borrow checker
/// mto puto. Fazer numa query só não dá porque nem tudo tem velocidade e tentar fazer uma sub-query
/// usando Query::transmute_lens_filtered também deixa o borrow checker puto
pub fn handle_collision_moving_static(
    mut query_moving: Query<
        (&mut Position, &Rectangle, &mut Velocity, Option<&mut Jump>),
        With<Solid>,
    >,
    mut query_static: Query<(&Position, &Rectangle), (With<Solid>, Without<Velocity>)>,
) {
    for (mut pos, rec, mut vel, mut jump) in query_moving.iter_mut() {
        let mut hitbox = rec.on_position_mut(&mut pos);
        for (pos, rec) in query_static.iter_mut() {
            let static_hitbox = rec.on_position(pos);
            if let Some(axis) = hitbox.colides_with_axis(&static_hitbox) {
                match axis {
                    CollisionAxis::Up => {
                        vel.y = 0.0;
                        hitbox.pos.y = static_hitbox.bottom() - hitbox.rect.height as f64;
                    }
                    CollisionAxis::Down => {
                        vel.y = 0.0;
                        hitbox.pos.y = static_hitbox.top();
                        if let Some(jump) = &mut jump {
                            jump.grounded = true;
                        }
                    }
                    CollisionAxis::Left => {
                        vel.x = 0.0;
                        hitbox.pos.x = static_hitbox.right();
                    }
                    CollisionAxis::Right => {
                        vel.x = 0.0;
                        hitbox.pos.x = static_hitbox.left() - hitbox.rect.width as f64;
                    }
                }
            }
        }
    }
}

//TODO this could probably be implemented inside handle_collision_moving_static as the code is basicaly equal.
pub fn handle_bounce_moving_static(
    mut query_moving: Query<
        (
            &mut Position,
            &Rectangle,
            &mut Velocity,
            Option<&mut Jump>,
            &Bounce,
        ),
        With<Solid>,
    >,
    mut query_static: Query<(&Position, &Rectangle), (With<Solid>, Without<Velocity>)>,
) {
    for (mut pos, rec, mut vel, mut jump, mut bounce) in query_moving.iter_mut() {
        if !bounce.enabled {
            println!("not bounced");
            continue;
        }
        let mut hitbox = rec.on_position_mut(&mut pos);
        for (pos, rec) in query_static.iter_mut() {
            let static_hitbox = rec.on_position(pos);
            if let Some(axis) = hitbox.colides_with_axis(&static_hitbox) {
                println!("bounced yeahhh!");
                match axis {
                    CollisionAxis::Up => {
                        vel.y = vel.y * -1.0 * bounce.bounciness;

                        hitbox.pos.y = static_hitbox.bottom() - hitbox.rect.height as f64;
                    }
                    CollisionAxis::Down => {
                        vel.y = vel.y * -1.0 * bounce.bounciness;

                        hitbox.pos.y = static_hitbox.top();
                    }
                    CollisionAxis::Left => {
                        vel.x = vel.x * -1.0 * bounce.bounciness;
                        hitbox.pos.x = static_hitbox.right();
                    }
                    CollisionAxis::Right => {
                        vel.x = vel.x * -1.0 * bounce.bounciness;
                        hitbox.pos.x = static_hitbox.left() - hitbox.rect.width as f64;
                    }
                }
            }
        }
    }
}

pub fn gravitate(mut query: Query<&mut Velocity, With<Gravitable>>, time: Res<Time>) {
    let delta = time.delta().as_secs_f64();
    for mut velocity in query.iter_mut() {
        if velocity.y > -PLAYER_MAX_VERTICAL_SPEED {
            velocity.y -= PLAYER_VERTICAL_ACCELERATION * delta;
        }
    }
}

pub fn limit_velocity(mut query: Query<&mut Velocity>) {
    const NEGATIVE_PLAYER_MAX_VERTICAL_SPEED: f64 = -PLAYER_MAX_VERTICAL_SPEED;
    const NEGATIVE_PLAYER_MAX_HORIZONTAL_SPEED: f64 = -PLAYER_MAX_HORIZONTAL_SPEED;
    for mut velocity in query.iter_mut() {
        velocity.y = match velocity.y {
            PLAYER_MAX_VERTICAL_SPEED.. => PLAYER_MAX_VERTICAL_SPEED,
            ..NEGATIVE_PLAYER_MAX_VERTICAL_SPEED => NEGATIVE_PLAYER_MAX_VERTICAL_SPEED,
            _ => velocity.y,
        };
        velocity.x = match velocity.x {
            PLAYER_MAX_HORIZONTAL_SPEED.. => PLAYER_MAX_HORIZONTAL_SPEED,
            ..NEGATIVE_PLAYER_MAX_HORIZONTAL_SPEED => NEGATIVE_PLAYER_MAX_HORIZONTAL_SPEED,
            _ => velocity.x,
        }
    }
}

pub fn move_system(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let delta = time.delta().as_secs_f64();
    for (mut pos, vel) in query.iter_mut() {
        pos.x += vel.x * delta;
        pos.y += vel.y * delta;
    }
}
