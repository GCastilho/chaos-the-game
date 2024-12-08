use crate::game::components::{
    hitbox::{HitboxBorrowedMut, RectInPosition, ToHitbox},
    CollisionAxis, Hitbox, Player, Position, Rectangle,
};
use bevy_ecs::{
    component::Component,
    query::{With, Without},
    system::{Query, ResMut, Resource},
    world::{FromWorld, World},
};
use sdl2::render::WindowCanvas;

#[derive(Component)]
pub struct CameraHitbox;

#[derive(Debug, Resource)]
pub struct Camera {
    pub pos: Position,
    pub rect: Rectangle,
}

impl Camera {
    pub fn hitbox(&mut self) -> Hitbox<HitboxBorrowedMut> {
        self.rect.on_position_mut(&mut self.pos)
    }
}

impl FromWorld for Camera {
    fn from_world(world: &mut World) -> Self {
        let (w, h) = world
            .non_send_resource_mut::<WindowCanvas>()
            .window()
            .size();

        // TODO: Dá pra adicionar o scheduler no world. Pode ajudar a organizar melhor
        Camera {
            pos: Position::new(0, 0),
            rect: Rectangle::new(w / 2, h - 20),
        }
    }
}

trait ColidesInverted<T: RectInPosition> {
    fn colides_with_axis_inverted<R: RectInPosition>(
        &self,
        other: &Hitbox<R>,
    ) -> Option<CollisionAxis>;
}

impl<T: RectInPosition> ColidesInverted<T> for Hitbox<T> {
    fn colides_with_axis_inverted<R: RectInPosition>(
        &self,
        other: &Hitbox<R>,
    ) -> Option<CollisionAxis> {
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

pub fn move_camera(
    mut camera: ResMut<Camera>,
    // mut camera_hitbox_query: Query<(&mut Position, &Rectangle), With<CameraHitbox>>,
    player: Query<(&Position, &Rectangle), (With<Player>, Without<CameraHitbox>)>,
) {
    let player = player.single().hitbox();
    let mut camera = camera.hitbox();

    let Some(axis) = player.colides_with_axis_inverted(&camera) else {
        return;
    };

    let delta = match axis {
        CollisionAxis::Up => (0.0, player.top() - camera.top()),
        CollisionAxis::Down => (0.0, player.bottom() - camera.bottom()),
        CollisionAxis::Left => (player.left() - camera.left(), 0.0),
        CollisionAxis::Right => (player.right() - camera.right(), 0.0),
    };
    println!("delta: {:?}", delta);

    camera.pos.x += delta.0;
    camera.pos.y += delta.1;
    println!("Camera moved: {:?}", camera.pos);
}
