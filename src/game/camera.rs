use crate::game::components::{CollisionAxis, Componentable, Hitbox, Player, Position, Rectangle};
use bevy_ecs::{
    component::Component,
    query::{With, Without},
    system::{Query, Resource},
    world::{FromWorld, World},
};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

#[derive(Debug, Component, Resource)]
pub struct Camera;

impl FromWorld for Camera {
    fn from_world(world: &mut World) -> Self {
        let (w, h) = world
            .non_send_resource_mut::<WindowCanvas>()
            .window()
            .size();
        world.spawn((
            Camera,
            Position::new(10, 10),
            Rectangle::new(w / 2, h - 20),
            Color::GREEN.into_outline(),
        ));
        // TODO: Dá pra adicionar o scheduler no world. Pode ajudar a organizar melhor
        Camera
    }
}

trait ColidesInverted {
    fn colides_with_axis_inverted(&self, other: &Hitbox) -> Option<CollisionAxis>;
}

impl ColidesInverted for Hitbox<'_> {
    fn colides_with_axis_inverted(&self, other: &Hitbox) -> Option<CollisionAxis> {
        if self.right() > other.right() {
            Some(CollisionAxis::Right)
        } else if self.left() < other.left() {
            Some(CollisionAxis::Left)
        } else if self.bottom() < other.bottom() {
            Some(CollisionAxis::Down)
        } else if self.top() > other.top() {
            Some(CollisionAxis::Up)
        } else {
            None
        }
    }
}

pub fn move_world(
    mut camera: Query<(&mut Position, &Rectangle), With<Camera>>,
    mut player: Query<(&mut Position, &Rectangle), (With<Player>, Without<Camera>)>,
    mut query: Query<&mut Position, (Without<Camera>, Without<Player>)>,
) {
    let (mut pos, rect) = player.single_mut();
    let player = rect.on_position(&mut pos);
    let (mut pos, rect) = camera.single_mut();
    let camera = rect.on_position(&mut pos);

    let Some(axis) = player.colides_with_axis_inverted(&camera) else {
        return;
    };

    let delta = match axis {
        CollisionAxis::Up => (0.0, camera.top() - player.top()),
        CollisionAxis::Down => (0.0, camera.bottom() - player.bottom()),
        CollisionAxis::Left => (camera.left() - player.left(), 0.0),
        CollisionAxis::Right => (camera.right() - player.right(), 0.0),
    };

    player.pos.x += delta.0;
    player.pos.y += delta.1;

    for mut pos in query.iter_mut() {
        pos.x += delta.0;
        pos.y += delta.1;
    }
}
