use crate::game::components::{CollisionAxis, Hitbox, Player, Position, Rectangle};
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
    pub fn hitbox(&mut self) -> Hitbox {
        self.rect.on_position(&mut self.pos)
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

pub fn move_camera(
    mut camera: ResMut<Camera>,
    // mut camera_hitbox_query: Query<(&mut Position, &Rectangle), With<CameraHitbox>>,
    mut player_query: Query<(&mut Position, &Rectangle), (With<Player>, Without<CameraHitbox>)>,
) {
    let (mut pos, rect) = player_query.single_mut();
    let player = rect.on_position(&mut pos);
    let camera = camera.hitbox();

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

pub fn move_world(
    mut camera: Query<(&mut Position, &Rectangle), With<CameraHitbox>>,
    mut player: Query<(&mut Position, &Rectangle), (With<Player>, Without<CameraHitbox>)>,
    mut query: Query<&mut Position, (Without<CameraHitbox>, Without<Player>)>,
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
