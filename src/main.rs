mod ecs;
mod game;
mod keyboard;

use bevy_ecs::{event::Events, prelude::Schedule, prelude::*, world::World};
use ecs::{
    draw_systems::{draw, Render},
    input::{update_input_state, InputEvent, InputState},
    physics_systems::{gravitate, handle_collision_moving_static, move_system},
    startup_systems::{init_player_system, Startup},
    Update,
};
use game::Game;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{rc::Rc, sync::Mutex, time::Duration};

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
        // .position(-900, 350)
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
    world.insert_resource(InputState::default());
    world.insert_resource(Events::<InputEvent>::default());

    Schedule::new(Startup)
        .add_systems(init_player_system)
        .run(&mut world);

    // TODO: Struct com todos os schedulers que roda todos automaticamente
    // E permite adicionar sistemas usando o nome do  scheduler
    let mut update_scheduler = Schedule::new(Update);
    update_scheduler
        .add_systems(update_input_state)
        .add_systems((gravitate, move_system, handle_collision_moving_static).chain());

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
                Event::KeyDown { repeat: false, .. } | Event::KeyUp { repeat: false, .. } => {
                    if let Ok(input_event) = keyboard::InputEvent::try_from(event.clone()) {
                        game.handle_keypress(input_event);
                    }
                    if let Ok(input_event) = InputEvent::try_from(event) {
                        println!("{:?}", input_event);
                        world.resource_mut::<Events<InputEvent>>().send(input_event);
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    game.handle_mousepress(x, canvas.lock().unwrap().window().size().1 as i32 - y);
                }
                Event::MouseButtonUp { x, y, .. } => {
                    game.handle_mouselift(x, canvas.lock().unwrap().window().size().1 as i32 - y);
                }
                Event::MouseMotion { x, y, .. } => {
                    println!("motion: ({x},{y})");
                }
                _ => {}
            }
        }

        game.update();
        game.draw(&mut canvas.lock().unwrap())?;
        update_scheduler.run(&mut world);
        render_scheduler.run(&mut world);

        canvas.lock().unwrap().present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
