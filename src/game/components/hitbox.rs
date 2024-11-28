use super::{CollisionAxis, Position, Rectangle};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    ops::{Deref, DerefMut},
};

pub trait RectInPosition {
    fn pos(&self) -> &Position;
    fn rect(&self) -> &Rectangle;
}

pub struct Hitbox<T: RectInPosition>(T);

impl<T: RectInPosition> Hitbox<T> {
    pub fn new(rect_in_position: T) -> Self {
        Hitbox(rect_in_position)
    }

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

impl<T: RectInPosition> Deref for Hitbox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: RectInPosition> DerefMut for Hitbox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct HitboxOwned {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hitbox_owned() {
        let owned = HitboxOwned {
            pos: Position::new(69, 420),
            rect: Rectangle::new(42, 55),
        };
        let hitbox = Hitbox(owned.clone());
        assert_eq!(hitbox.rect, owned.rect);
        assert_eq!(*hitbox.pos(), owned.pos);
    }

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
