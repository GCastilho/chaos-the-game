use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Up,
    Down,
}

#[derive(Debug)]
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

pub fn parse_keyboard_event(event: Event) -> Option<(Action, KeyState)> {
    match event {
        Event::KeyDown {
            keycode: Some(keycode),
            repeat: false,
            ..
        } => keycode
            .try_into()
            .ok()
            .map(|action| (action, KeyState::Down)),
        Event::KeyUp {
            keycode: Some(keycode),
            repeat: false,
            ..
        } => keycode.try_into().ok().map(|action| (action, KeyState::Up)),
        _ => None,
    }
}
