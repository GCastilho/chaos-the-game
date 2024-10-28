mod game;
mod keyboard;

use crate::game::Game;
use keyboard::InputEvent;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("Could not init SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");

    let window = video_subsystem
        .window("A Rust Game", SCREEN_WIDTH, SCREEN_HEIGHT)
        // .position_centered()
        .position(2200, 100)
        .build()
        .expect("Failed to build main window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to get SDL canvas");
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut game = Game::new(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT));

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");
    'running: loop {
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    game.handle_click(x as usize, y as usize);
                }
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    if let Ok(input_event) = InputEvent::try_from(event) {
                        println!("{input_event:?}");
                        game.handle_keypress(input_event);
                    }
                }
                _ => {}
            }
        }

        game.update();
        game.draw(&mut canvas)?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
