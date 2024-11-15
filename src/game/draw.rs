use super::components::{Color, Position, Rectangle};
use bevy_ecs::{
    schedule::ScheduleLabel,
    system::{NonSendMut, Query},
};
use sdl2::{rect::Rect, render::WindowCanvas};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Render;

// TODO: Não ter unwrap
pub fn draw(
    query: Query<(&Position, &Rectangle, &Color)>,
    canvas: NonSendMut<Rc<RefCell<WindowCanvas>>>,
) {
    let mut canvas = canvas.borrow_mut();
    for (pos, rect, color) in query.iter() {
        canvas.set_draw_color(**color);
        let square: Rect = Rect::new(
            pos.x,
            canvas.window().size().1 as i32 - pos.y - rect.height as i32,
            rect.width,
            rect.height,
        );
        canvas.fill_rect(square).unwrap();
    }
}
