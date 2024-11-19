use super::components::{CollisionAxis, Gravitable, Position, Rectangle, Solid, Velocity};
use crate::game::player::Jump;
use crate::game::resources::Time;
use bevy_ecs::prelude::{Query, Res};
use bevy_ecs::query::{With, Without};

pub const PLAYER_MAX_VERTICAL_SPEED: f64 = 15.0;
pub const PLAYER_VERTICAL_ACCELERATION: f64 = 1.0;

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
    mut query_static: Query<(&mut Position, &Rectangle), (With<Solid>, Without<Velocity>)>,
) {
    for (mut pos, rec, mut vel, mut jump) in query_moving.iter_mut() {
        let hitbox = rec.on_position(&mut *pos);
        for (mut pos, rec) in query_static.iter_mut() {
            let static_hitbox = rec.on_position(&mut *pos);
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

pub fn gravitate(mut query: Query<&mut Velocity, With<Gravitable>>, time: Res<Time>) {
    println!("gravitate: {:?} {:?}", time.delta(), time.elapsed());
    for mut velocity in query.iter_mut() {
        if velocity.y >= -PLAYER_MAX_VERTICAL_SPEED {
            velocity.y -= PLAYER_VERTICAL_ACCELERATION;
        }
    }
}

pub fn move_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}
