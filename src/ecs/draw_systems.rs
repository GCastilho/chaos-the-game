use super::components::{Position, Rectangle};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::{NonSendMut, Query};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Render;

// TODO: Não ter unwrap
pub fn draw(
    query: Query<(&Position, &Rectangle)>,
    mut canvas: NonSendMut<Rc<Mutex<WindowCanvas>>>,
) {
    let mut canvas = canvas.lock().unwrap();
    for (pos, rect) in query.iter() {
        canvas.set_draw_color(Color::BLUE);
        let square: Rect = Rect::new(
            pos.x,
            canvas.window().size().1 as i32 - pos.y - rect.height as i32,
            rect.width,
            rect.height,
        );
        canvas.fill_rect(square).unwrap();
    }
}
