mod game;

use crate::game::resources::Time;
use bevy_ecs::{event::Events, prelude::Schedule, prelude::*, world::World};
use game::{
    draw::{draw, Render},
    input::{
        handle_mouse, insert_mouse_resources, insert_mouse_square, update_input_state, InputEvent,
        InputState, MouseLift, MousePress,
    },
    physics::{gravitate, handle_collision_moving_static, move_system},
    player::{handle_player_input, player_collides_coin, update_jump_time},
    startup::{init_player_system, Startup},
    Update,
};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas};
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// TODO: Posição usar float e movimento não ser dependente do frame
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

    let canvas = window
        .into_canvas()
        .build()
        .expect("Failed to get SDL canvas");

    let mut world = World::new();

    world.insert_non_send_resource(canvas);
    world.insert_resource(InputState::default());
    world.insert_resource(Events::<InputEvent>::default());
    world.insert_resource(Time::new());
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
        let mut canvas = world.non_send_resource_mut::<WindowCanvas>();
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        canvas.clear();
        drop(canvas);
        world.resource_mut::<Time>().update();

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
                        .send(MousePress::new(x, SCREEN_HEIGHT as i32 - y));
                }
                Event::MouseButtonUp { x, y, .. } => {
                    world
                        .resource_mut::<Events<MouseLift>>()
                        .send(MouseLift::new(x, SCREEN_HEIGHT as i32 - y));
                }
                Event::MouseMotion { x, y, .. } => {
                    println!("motion: ({x},{y})");
                }
                _ => {}
            }
        }

        update_scheduler.run(&mut world);
        render_scheduler.run(&mut world);

        world.non_send_resource_mut::<WindowCanvas>().present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
