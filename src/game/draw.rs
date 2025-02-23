use super::{
    camera::Camera,
    components::{ColorDrawType, Colorable, Position, Rectangle},
};
use bevy_ecs::{
    schedule::ScheduleLabel,
    system::{NonSendMut, Query, ResMut},
};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{BlendMode, WindowCanvas},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Render;

pub fn draw(
    mut camera: ResMut<Camera>, // TODO: Usar Res qdo n desenhar mais hitbox
    query: Query<(&Position, &Rectangle, &Colorable)>,
    mut canvas: NonSendMut<WindowCanvas>,
) {
    for (pos, rect, colorable) in query.iter() {
        let square = Rect::new(
            pos.x as i32 - camera.pos.x.floor() as i32,
            canvas.window().size().1 as i32 - pos.y as i32 - rect.height as i32
                + camera.pos.y.floor() as i32,
            rect.width,
            rect.height,
        );
        canvas.set_draw_color(colorable.color);
        canvas.set_blend_mode(BlendMode::Blend);
        match colorable.draw_type {
            ColorDrawType::Fill => canvas.fill_rect(square).expect("Can't fill rect"),
            ColorDrawType::Outline => canvas.draw_rect(square).expect("Can't draw rect"),
        };
    }

    // TODO: Draw do hitbox da câmera deveria ser feito no for normal (só desenha se tem cor)
    let square = Rect::new(
        camera.pos.x as i32 - camera.hitbox().left() as i32,
        canvas.window().size().1 as i32 - camera.pos.y.round() as i32 - camera.rect.height as i32
            + camera.hitbox().bottom() as i32,
        camera.rect.width,
        camera.rect.height,
    );
    canvas.set_draw_color(Color::GREEN);
    canvas.draw_rect(square).expect("Can't draw rect");
}
