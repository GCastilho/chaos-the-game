use bevy_ecs::event::EventReader;
use bevy_ecs::system::{ResMut, Resource};
use enum_map::EnumMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Debug, Default, Copy, Clone)]
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
}

impl TryFrom<Keycode> for Action {
    type Error = Keycode;

    fn try_from(keycode: Keycode) -> Result<Self, Self::Error> {
        let action = match keycode {
            Keycode::Left => Self::Left,
            Keycode::Right => Self::Right,
            Keycode::Down => Self::Down,
            Keycode::Up => Self::Up,
            _ => return Err(keycode),
        };
        Ok(action)
    }
}

#[derive(Debug, bevy_ecs::event::Event)]
pub struct InputEvent {
    action: Action,
    state: ActionState,
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
