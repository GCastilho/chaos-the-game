use crate::game::components::{Componentable, Position, Rectangle};
use bevy_ecs::{
    component::Component,
    system::Resource,
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
        Camera
    }
}
