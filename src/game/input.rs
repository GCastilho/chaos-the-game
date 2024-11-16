use super::components::{Componentable, Position, Rectangle, Solid};
use bevy_ecs::{
    event::{EventReader, EventWriter, Events},
    prelude::Commands,
    system::{Local, ResMut, Resource},
    world::World,
};
use enum_map::EnumMap;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::cmp::{max, min};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ActionState {
    #[default]
    Inactive,
    Active,
}

impl ActionState {
    pub fn active(&self) -> bool {
        match self {
            ActionState::Inactive => false,
            ActionState::Active => true,
        }
    }
}

#[derive(Debug, enum_map::Enum, Eq, PartialEq, Copy, Clone)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Atack,
}

impl TryFrom<Keycode> for Action {
    type Error = Keycode;

    fn try_from(keycode: Keycode) -> Result<Self, Self::Error> {
        let action = match keycode {
            Keycode::Left => Self::Left,
            Keycode::Right => Self::Right,
            Keycode::Down => Self::Down,
            Keycode::Up => Self::Up,
            Keycode::A => Self::Atack,
            _ => return Err(keycode),
        };
        Ok(action)
    }
}

#[derive(Debug, bevy_ecs::event::Event)]
pub struct InputEvent {
    pub action: Action,
    pub state: ActionState,
}

impl TryFrom<Event> for InputEvent {
    type Error = Event;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => keycode
                .try_into()
                .map(|action| Self {
                    action,
                    state: ActionState::Active,
                })
                .map_err(|_| event),
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => keycode
                .try_into()
                .map(|action| Self {
                    action,
                    state: ActionState::Inactive,
                })
                .map_err(|_| event),
            _ => Err(event),
        }
    }
}

#[derive(Debug, Resource, Default)]
pub struct InputState {
    state: EnumMap<Action, ActionState>,
}

impl InputState {
    pub fn state(&self) -> &EnumMap<Action, ActionState> {
        &self.state
    }
}

pub fn update_input_state(
    mut ev_input: EventReader<InputEvent>,
    mut input_state: ResMut<InputState>,
) {
    for ev in ev_input.read() {
        input_state.state[ev.action] = ev.state;
    }
}

#[derive(Debug, bevy_ecs::event::Event, Copy, Clone)]
pub struct MousePress {
    x: i32,
    y: i32,
}

impl MousePress {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, bevy_ecs::event::Event, Copy, Clone)]
pub struct MouseLift {
    x: i32,
    y: i32,
}

impl MouseLift {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, bevy_ecs::event::Event)]
pub struct MouseCommand {
    pub press: MousePress,
    pub lift: MouseLift,
}

pub fn handle_mouse(
    mut last_press: Local<Option<MousePress>>,
    mut ev_pressed: EventReader<MousePress>,
    mut ev_lift: EventReader<MouseLift>,
    mut mouse_command: EventWriter<MouseCommand>,
) {
    for press in ev_pressed.read() {
        last_press.replace(*press);
    }
    for lift in ev_lift.read() {
        if let Some(press) = last_press.take() {
            mouse_command.send(MouseCommand { press, lift: *lift });
        }
    }
}

pub fn insert_mouse_resources(world: &mut World) {
    world.insert_resource(Events::<MousePress>::default());
    world.insert_resource(Events::<MouseLift>::default());
    world.insert_resource(Events::<MouseCommand>::default());
}

pub fn insert_mouse_square(mut mouse_commands: EventReader<MouseCommand>, mut commands: Commands) {
    for mouse_command in mouse_commands.read() {
        let min_x = min(mouse_command.lift.x, mouse_command.press.x);
        let min_y = min(mouse_command.lift.y, mouse_command.press.y);

        let max_x = max(mouse_command.lift.x, mouse_command.press.x);
        let max_y = max(mouse_command.lift.y, mouse_command.press.y);

        commands.spawn((
            Position::new(min_x, min_y),
            Rectangle::new((max_x - min_x) as u32, (max_y - min_y) as u32),
            Color::CYAN.into_component(),
            Solid::all(),
        ));
    }
}
