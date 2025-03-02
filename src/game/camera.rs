use crate::game::components::{
    hitbox::{HitboxBorrowedMut, RectInPosition, ToHitbox},
    CollisionAxis, ColorDrawType, Colorable, Hitbox, Player, Position, Rectangle,
};
use bevy_ecs::{
    component::Component,
    query::{With, Without},
    system::{Query, ResMut, Resource},
    world::{FromWorld, World},
};
use sdl2::{pixels::Color, render::WindowCanvas};

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

        world.spawn((
            CameraHitbox,
            Margin {
                top: 10.0,
                right: (w / 2) as f64,
                bottom: 0.0,
                left: 10.0,
            }
            .into_bundle((w, h)),
            Colorable::new(Color::GREEN, ColorDrawType::Outline),
        ));

        // TODO: Dá pra adicionar o scheduler no world. Pode ajudar a organizar melhor
        Camera {
            pos: Position::new(0, 0),
            rect: Rectangle::new(w, h),
        }
    }
}

struct HitboxOwned {
    pos: Position,
    rect: Rectangle,
}

impl RectInPosition for HitboxOwned {
    fn pos(&self) -> &Position {
        &self.pos
    }

    fn rect(&self) -> &Rectangle {
        &self.rect
    }
}

#[derive(Debug, Component)]
pub struct Margin {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

impl Margin {
    /// Retorna um bundle (para o bevy) da position e rectangle calculada usando a window e margin
    fn into_bundle(self, (w, h): (u32, u32)) -> (Position, Rectangle, Self) {
        let pos = Position::new(self.left, self.bottom);
        let rect = Rectangle::new(
            ((w as f64) - self.left - self.right) as u32,
            ((h as f64) - self.top - self.bottom) as u32,
        );
        (pos, rect, self)
    }
}

trait ColidesInverted<T: RectInPosition> {
    fn colides_with_axis_inverted<R: RectInPosition>(
        &self,
        other: &Hitbox<R>,
    ) -> Option<CollisionAxis>;

    fn with_margin(&self, margin: &Margin) -> Hitbox<HitboxOwned>;
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

    /// Retorna uma a hitbox de self + margem
    fn with_margin(&self, margin: &Margin) -> Hitbox<HitboxOwned> {
        let pos = Position::new(self.left() - margin.left, self.bottom() - margin.bottom);
        let rect = Rectangle::new(
            (self.rect().width as f64 + margin.right + margin.left) as u32,
            (self.rect().height as f64 + margin.bottom + margin.top) as u32,
        );
        Hitbox::new(HitboxOwned { pos, rect })
    }
}

pub fn move_camera(
    mut camera: ResMut<Camera>,
    mut camera_hitbox: Query<
        (&mut Position, &Rectangle, &Margin),
        (Without<Player>, With<CameraHitbox>),
    >,
    player: Query<(&Position, &Rectangle), (With<Player>, Without<CameraHitbox>)>,
) {
    let player = player.single().hitbox();
    let (mut pos, rect, margin) = camera_hitbox.single_mut();
    let mut camera_hitbox = rect.on_position_mut(&mut pos);

    let Some(axis) = player.colides_with_axis_inverted(&camera_hitbox) else {
        return;
    };

    let delta = match axis {
        CollisionAxis::Up => (0.0, player.top() - camera_hitbox.top()),
        CollisionAxis::Down => (0.0, player.bottom() - camera_hitbox.bottom()),
        CollisionAxis::Left => (player.left() - camera_hitbox.left(), 0.0),
        CollisionAxis::Right => (player.right() - camera_hitbox.right(), 0.0),
    };
    camera_hitbox.pos.x += delta.0;
    camera_hitbox.pos.y += delta.1;
    println!("Camera hitbox moved: {:?}", camera_hitbox.pos);

    let mut camera = camera.hitbox();
    let margin_hitbox = camera_hitbox.with_margin(&margin);
    let Some(axis) = margin_hitbox.colides_with_axis_inverted(&camera) else {
        return;
    };

    let delta = match axis {
        CollisionAxis::Up => (0.0, margin_hitbox.top() - camera.top()),
        CollisionAxis::Down => (0.0, margin_hitbox.bottom() - camera.bottom()),
        CollisionAxis::Left => (margin_hitbox.left() - camera.left(), 0.0),
        CollisionAxis::Right => (margin_hitbox.right() - camera.right(), 0.0),
    };
    camera.pos.x += delta.0;
    camera.pos.y += delta.1;
    println!("Camera moved: {:?}", camera.pos);
}
