use enum_map::EnumMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyState {
    Down,
    #[default]
    Up,
}

#[derive(Debug, Clone, Copy, enum_map::Enum, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<Keycode> for Action {
    type Error = ();

    fn try_from(keycode: Keycode) -> Result<Self, Self::Error> {
        let action = match keycode {
            Keycode::Left => Self::Left,
            Keycode::Right => Self::Right,
            Keycode::Down => Self::Down,
            Keycode::Up => Self::Up,
            _ => return Err(()),
        };
        Ok(action)
    }
}

#[derive(Debug)]
pub struct InputEvent {
    pub action: Action,
    pub key_state: KeyState,
}

impl InputEvent {
    pub fn is(&self, action: Action, key_state: KeyState) -> bool {
        self.action == action && self.key_state == key_state
    }
}

impl TryFrom<Event> for InputEvent {
    type Error = ();

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => keycode.try_into().map(|action| Self {
                key_state: KeyState::Down,
                action,
            }),
            Event::KeyUp {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => keycode.try_into().map(|action| Self {
                key_state: KeyState::Up,
                action,
            }),
            _ => Err(()),
        }
    }
}

pub struct InputController {
    pub state: EnumMap<Action, KeyState>,
    pub last_event: InputEvent, // Todo: Faz mais sentido ser um VecDupe/Option
}

impl InputController {
    pub fn new() -> Self {
        let dummy_event = InputEvent {
            action: Action::Right,
            key_state: KeyState::Down,
        };
        Self {
            state: EnumMap::default(),
            last_event: dummy_event,
        }
    }

    pub fn handle_input_event(&mut self, input_event: InputEvent) {
        self.state[input_event.action] = input_event.key_state;
        self.last_event = input_event;
    }
}
