use super::components::{CollisionAxis, Position, Rectangle, Solid, Velocity};
use bevy_ecs::prelude::Query;
use bevy_ecs::query::{With, Without};
use bevy_ecs::world::Mut;
use std::cmp::Ordering::{Equal, Greater, Less};

const PLAYER_VERTICAL_ACCELERATION: i32 = 1;

// Trait temporária pra poder implementar método no rect do outro módulo
pub trait ToHitbox {
    fn on_position_bevy<'a>(&'a self, position: Mut<'a, Position>) -> Hitbox<'a>;
}

impl ToHitbox for Rectangle {
    fn on_position_bevy<'a>(&'a self, position: Mut<'a, Position>) -> Hitbox<'a> {
        Hitbox {
            rect: self,
            pos: position,
        }
    }
}

pub struct Hitbox<'a> {
    pos: Mut<'a, Position>,
    rect: &'a Rectangle,
}

impl<'a> Hitbox<'a> {
    pub fn left(&self) -> i32 {
        self.pos.x
    }

    pub fn right(&self) -> i32 {
        self.pos.x + self.rect.width as i32
    }

    pub fn top(&self) -> i32 {
        self.pos.y + self.rect.height as i32
    }

    pub fn bottom(&self) -> i32 {
        self.pos.y
    }

    pub fn colides_with(&self, other: &'a Hitbox<'a>) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.bottom() < other.top()
            && self.top() > other.bottom()
    }

    pub fn colides_with_axis(&self, other: &Hitbox) -> Option<CollisionAxis> {
        if !self.colides_with(other) {
            return None;
        }

        let y_up = self.top() - other.bottom();
        let y_down = other.top() - self.bottom();
        let x_right = self.right() - other.left();
        let x_left = other.right() - self.left();

        let (y_axis, y_value) = match y_up.cmp(&y_down) {
            Greater | Equal => (CollisionAxis::Down, y_down),
            Less => (CollisionAxis::Up, y_up),
        };

        let (x_axis, x_value) = match x_left.cmp(&x_right) {
            Greater | Equal => (CollisionAxis::Right, x_right),
            Less => (CollisionAxis::Left, x_left),
        };

        match y_value.cmp(&x_value) {
            Greater => Some(x_axis),
            Less => Some(y_axis),
            Equal => None,
        }
    }
}

/// Colisão entre coisas com e sem velocidade.
///
/// Não dá para fazer todas as colisões aqui porque elas dão overlap, e isso deixa o borrow checker
/// mto puto. Fazer numa query só não dá porque nem tudo tem velocidade e tentar fazer uma sub-query
/// usando Query::transmute_lens_filtered também deixa o borrow checker puto
pub fn handle_collision_moving_static(
    mut query_moving: Query<(&mut Position, &Rectangle, &mut Velocity), With<Solid>>,
    mut query_static: Query<(&mut Position, &Rectangle), (With<Solid>, Without<Velocity>)>,
) {
    for (mut pos, rec, mut vel) in query_moving.iter_mut() {
        let mut hitbox = rec.on_position_bevy(pos.reborrow());
        for (mut pos, rec) in query_static.iter_mut() {
            let static_hitbox = rec.on_position_bevy(pos.reborrow());
            if let Some(axis) = hitbox.colides_with_axis(&static_hitbox) {
                match axis {
                    CollisionAxis::Up => {
                        vel.y = 0;
                        hitbox.pos.y = static_hitbox.bottom() - hitbox.rect.height as i32;
                    }
                    CollisionAxis::Down => {
                        vel.y = 0;
                        hitbox.pos.y = static_hitbox.top();
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
