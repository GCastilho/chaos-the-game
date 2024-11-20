use super::components::{ColorDrawType, Colorable, Position, Rectangle};
use bevy_ecs::{
    schedule::ScheduleLabel,
    system::{NonSendMut, Query},
};
use sdl2::{rect::Rect, render::WindowCanvas};

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Render;

pub fn draw(
    query: Query<(&Position, &Rectangle, &Colorable)>,
    mut canvas: NonSendMut<WindowCanvas>,
) {
    for (pos, rect, colorable) in query.iter() {
        canvas.set_draw_color(colorable.color);
        let square: Rect = Rect::new(
            pos.x as i32,
            canvas.window().size().1 as i32 - pos.y as i32 - rect.height as i32,
            rect.width,
            rect.height,
        );
        match colorable.draw_type {
            ColorDrawType::Fill => canvas.fill_rect(square).expect("Can't fill rect"),
            ColorDrawType::Outline => canvas.draw_rect(square).expect("Can't draw rect"),
        };
    }
}
