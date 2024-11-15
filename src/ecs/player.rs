use crate::ecs::{
    components::{CoinKind, Color, Player, Position, Rectangle, Velocity},
    physics_systems::ToHitbox,
};
use bevy_ecs::query::Without;
use bevy_ecs::{prelude::Query, query::With};

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
