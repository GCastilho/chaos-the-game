mod ecs;
mod game;
mod keyboard;

use crate::ecs::draw_systems::{draw, Render};
use crate::ecs::startup_systems::{init_player_system, Startup};
use crate::game::Game;
use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use keyboard::InputEvent;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::rc::Rc;
use std::sync::Mutex;
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
        .position_centered()
        .position(-900, 350)
        .build()
        .expect("Failed to build main window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to get SDL canvas");
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut game = Game::new();
    let mut world = World::new();

    // TODO: Canvas está temporariamente num mutex até finalização da migração para o bevy_ecs
    let canvas = Rc::new(Mutex::new(canvas));

    world.insert_non_send_resource(canvas.clone());

    Schedule::new(Startup)
        .add_systems(init_player_system)
        .run(&mut world);

    let mut render_scheduler = Schedule::new(Render);

    render_scheduler.add_systems(draw);

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");
    'running: loop {
        canvas
            .lock()
            .unwrap()
            .set_draw_color(Color::RGB(80, 80, 80));
        canvas.lock().unwrap().clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    if let Ok(input_event) = InputEvent::try_from(event) {
                        println!("{input_event:?}");
                        game.handle_keypress(input_event);
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    game.handle_mousepress(x, canvas.lock().unwrap().window().size().1 as i32 - y);
                }
                Event::MouseButtonUp { x, y, .. } => {
                    game.handle_mouselift(
                        x as i32,
                        canvas.lock().unwrap().window().size().1 as i32 - y,
                    );
                }
                Event::MouseMotion { x, y, .. } => {
                    println!("motion: ({x},{y})");
                }
                _ => {}
            }
        }

        game.update();
        game.draw(&mut canvas.lock().unwrap())?;
        render_scheduler.run(&mut world);

        canvas.lock().unwrap().present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
