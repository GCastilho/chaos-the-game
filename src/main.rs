mod game;

use crate::game::input::{
    handle_mouse, handle_player_input, insert_mouse_resources, insert_mouse_square, MouseLift,
    MousePress,
};
use crate::game::player::{player_collides_coin, update_jump_time};
use bevy_ecs::{event::Events, prelude::Schedule, prelude::*, world::World};
use game::{
    draw_systems::{draw, Render},
    input::{update_input_state, InputEvent, InputState},
    physics_systems::{gravitate, handle_collision_moving_static, move_system},
    startup_systems::{init_player_system, Startup},
    Update,
};
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

    let mut world = World::new();

    // TODO: Tentar tirar canvas de um mutex
    let canvas = Rc::new(Mutex::new(canvas));

    world.insert_non_send_resource(canvas.clone());
    world.insert_resource(InputState::default());
    world.insert_resource(Events::<InputEvent>::default());
    insert_mouse_resources(&mut world);

    Schedule::new(Startup)
        .add_systems(init_player_system)
        .run(&mut world);

    // TODO: Struct com todos os schedulers que roda todos automaticamente
    // E permite adicionar sistemas usando o nome do  scheduler
    let mut update_scheduler = Schedule::new(Update);
    update_scheduler
        .add_systems((update_input_state, handle_player_input).chain())
        .add_systems(
            (gravitate, move_system, handle_collision_moving_static)
                .chain()
                .after(update_input_state),
        )
        .add_systems(player_collides_coin)
        .add_systems(update_jump_time)
        .add_systems((handle_mouse, insert_mouse_square));

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
                    if let Ok(input_event) = InputEvent::try_from(event) {
                        println!("{:?}", input_event);
                        world.resource_mut::<Events<InputEvent>>().send(input_event);
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    world
                        .resource_mut::<Events<MousePress>>()
                        .send(MousePress::new(
                            x,
                            canvas.lock().unwrap().window().size().1 as i32 - y,
                        ));
                }
                Event::MouseButtonUp { x, y, .. } => {
                    world
                        .resource_mut::<Events<MouseLift>>()
                        .send(MouseLift::new(
                            x,
                            canvas.lock().unwrap().window().size().1 as i32 - y,
                        ));
                }
                Event::MouseMotion { x, y, .. } => {
                    println!("motion: ({x},{y})");
                }
                _ => {}
            }
        }

        update_scheduler.run(&mut world);
        render_scheduler.run(&mut world);

        canvas.lock().unwrap().present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
