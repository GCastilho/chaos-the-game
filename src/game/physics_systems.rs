use super::components::{CollisionAxis, Position, Rectangle, Solid, Velocity};
use crate::game::player::Jump;
use bevy_ecs::prelude::Query;
use bevy_ecs::query::{With, Without};

const PLAYER_VERTICAL_ACCELERATION: i32 = 1;

/// Colisão entre coisas com e sem velocidade.
///
/// Não dá para fazer todas as colisões aqui porque elas dão overlap, e isso deixa o borrow checker
/// mto puto. Fazer numa query só não dá porque nem tudo tem velocidade e tentar fazer uma sub-query
/// usando Query::transmute_lens_filtered também deixa o borrow checker puto
pub fn handle_collision_moving_static(
    mut query_moving: Query<(&mut Position, &Rectangle, &mut Velocity, &mut Jump), With<Solid>>,
    mut query_static: Query<(&mut Position, &Rectangle), (With<Solid>, Without<Velocity>)>,
) {
    for (mut pos, rec, mut vel, mut jump) in query_moving.iter_mut() {
        let mut hitbox = rec.on_position(pos.reborrow());
        for (mut pos, rec) in query_static.iter_mut() {
            let static_hitbox = rec.on_position(pos.reborrow());
            if let Some(axis) = hitbox.colides_with_axis(&static_hitbox) {
                match axis {
                    CollisionAxis::Up => {
                        vel.y = 0;
                        hitbox.pos.y = static_hitbox.bottom() - hitbox.rect.height as i32;
                    }
                    CollisionAxis::Down => {
                        vel.y = 0;
                        hitbox.pos.y = static_hitbox.top();
                        jump.grounded = true;
                    }
                    CollisionAxis::Left => {
                        vel.x = 0;
                        hitbox.pos.x = static_hitbox.right();
                    }
                    CollisionAxis::Right => {
                        vel.x = 0;
                        hitbox.pos.x = static_hitbox.left() - hitbox.rect.width as i32;
                    }
                }
            }
        }
    }
}

pub fn gravitate(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.y -= PLAYER_VERTICAL_ACCELERATION;
    }
}

pub fn move_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}
