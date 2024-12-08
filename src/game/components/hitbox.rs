use super::{Position, Rectangle};
use bevy_ecs::{change_detection::Mut, entity::Entity};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub enum CollisionAxis {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Hitbox<T>(T);

impl<T> Hitbox<T> {
    pub fn new(hitbox: T) -> Self {
        Hitbox(hitbox)
    }
}

impl<T> Deref for Hitbox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Hitbox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait RectInPosition {
    fn pos(&self) -> &Position;
    fn rect(&self) -> &Rectangle;
}

impl<T: RectInPosition> Hitbox<T> {
    pub fn left(&self) -> f64 {
        self.pos().x
    }

    pub fn right(&self) -> f64 {
        self.pos().x + self.rect().width as f64
    }

    pub fn top(&self) -> f64 {
        self.pos().y + self.rect().height as f64
    }

    pub fn bottom(&self) -> f64 {
        self.pos().y
    }

    pub fn center(&self) -> Position {
        let x = self.pos().x + (self.rect().width as f64 / 2.0);
        let y = self.pos().y + (self.rect().height as f64 / 2.0);
        Position { x, y }
    }

    pub fn colides_with<R: RectInPosition>(&self, other: &Hitbox<R>) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.bottom() < other.top()
            && self.top() > other.bottom()
    }

    pub fn colides_with_axis<R: RectInPosition>(&self, other: &Hitbox<R>) -> Option<CollisionAxis> {
        if !self.colides_with(other) {
            return None;
        }

        let y_up = self.top() - other.bottom();
        let y_down = other.top() - self.bottom();
        let x_right = self.right() - other.left();
        let x_left = other.right() - self.left();

        let (y_axis, y_value) = match y_up.total_cmp(&y_down) {
            Greater | Equal => (CollisionAxis::Down, y_down),
            Less => (CollisionAxis::Up, y_up),
        };

        let (x_axis, x_value) = match x_left.total_cmp(&x_right) {
            Greater | Equal => (CollisionAxis::Right, x_right),
            Less => (CollisionAxis::Left, x_left),
        };

        match y_value.total_cmp(&x_value) {
            Greater => Some(x_axis),
            Less => Some(y_axis),
            Equal => None,
        }
    }
}

#[derive(Debug)]
pub struct HitboxOwned<'a> {
    pub pos: Mut<'a, Position>,
    pub rect: &'a Rectangle,
}

impl<'a> RectInPosition for HitboxOwned<'a> {
    fn pos(&self) -> &Position {
        &self.pos
    }

    fn rect(&self) -> &Rectangle {
        &self.rect
    }
}

#[derive(Debug, Clone)]
pub struct HitboxBorrowed<'a> {
    pub pos: &'a Position,
    pub rect: &'a Rectangle,
}

impl<'a> RectInPosition for HitboxBorrowed<'a> {
    fn pos(&self) -> &Position {
        self.pos
    }

    fn rect(&self) -> &Rectangle {
        self.rect
    }
}

#[derive(Debug)]
pub struct HitboxBorrowedMut<'a> {
    pub pos: &'a mut Position,
    pub rect: &'a Rectangle,
}

impl<'a> RectInPosition for HitboxBorrowedMut<'a> {
    fn pos(&self) -> &Position {
        self.pos
    }

    fn rect(&self) -> &Rectangle {
        self.rect
    }
}

pub trait ToHitbox<T> {
    fn hitbox(&self) -> Hitbox<T>;
}

pub trait ToHitboxMut<'a> {
    fn hitbox_mut(&'a mut self) -> Hitbox<HitboxBorrowedMut<'a>>;
}

pub trait IntoHitbox<T> {
    fn into_hitbox(self) -> Hitbox<T>;
}

impl<'a> ToHitbox<HitboxBorrowed<'a>> for (&'a Position, &'a Rectangle) {
    fn hitbox(&self) -> Hitbox<HitboxBorrowed<'a>> {
        let (pos, rect) = *self;
        Hitbox(HitboxBorrowed { pos, rect })
    }
}

impl<'a> ToHitboxMut<'a> for (Mut<'a, Position>, &'a Rectangle) {
    fn hitbox_mut(&'a mut self) -> Hitbox<HitboxBorrowedMut<'a>> {
        let (ref mut pos, rect) = self;
        Hitbox(HitboxBorrowedMut { pos, rect })
    }
}

impl<'a> IntoHitbox<HitboxOwned<'a>> for (Mut<'a, Position>, &'a Rectangle) {
    fn into_hitbox(self) -> Hitbox<HitboxOwned<'a>> {
        let (pos, rect) = self;
        Hitbox(HitboxOwned { pos, rect })
    }
}

impl<'a> ToHitbox<HitboxBorrowed<'a>> for (Entity, &'a Position, &'a Rectangle) {
    fn hitbox(&self) -> Hitbox<HitboxBorrowed<'a>> {
        let (_, pos, rect) = *self;
        Hitbox(HitboxBorrowed { pos, rect })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hitbox_borrowed() {
        let pos = Position::new(69, 420);
        let rect = Rectangle::new(42, 55);
        let borrowed = HitboxBorrowed {
            pos: &pos,
            rect: &rect,
        };
        let hitbox = Hitbox(borrowed.clone());
        assert_eq!(hitbox.rect(), borrowed.rect);
        assert_eq!(hitbox.pos, borrowed.pos);
    }

    #[test]
    fn hitbox_borrowed_mut() {
        let mut pos = Position::new(69, 420);
        let rect = Rectangle::new(42, 55);
        let borrowed_mut = HitboxBorrowedMut {
            pos: &mut pos,
            rect: &rect,
        };
        let mut hitbox = Hitbox(borrowed_mut);
        assert_eq!(*hitbox.rect(), rect);
        hitbox.pos.x += 10.0;
        assert_eq!(*hitbox.pos(), Position::new(79, 420));
    }
}
