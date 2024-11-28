use super::{
    components::{
        CoinKind, Componentable, Gravitable, Player, Position, Rectangle, Solid, Velocity,
    },
    physics::PLAYER_VERTICAL_ACCELERATION,
    player::Jump,
};
use bevy_ecs::{schedule::ScheduleLabel, system::Commands};
use sdl2::pixels::Color;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Startup;

pub fn init_player_system(mut commands: Commands) {
    commands.spawn((
        Player,
        Position::new(250, 600),
        Rectangle::new(50, 50),
        Velocity::default(),
        Color::BLUE.into_fill(),
        Solid::all(),
        Jump::default(),
        Gravitable,
    ));

    // TODO: Chão não ser inicializado aqui
    commands.spawn((
        Position::new(100, 100),
        Rectangle::new(400, 10),
        Color::GREEN.into_fill(),
        Solid::all(),
    ));

    let coin_rect = Rectangle {
        width: 10,
        height: 10,
    };

    // TODO: Coins não serem inicializadas aqui
    commands.spawn((
        Position::new(120, 115),
        coin_rect,
        CoinKind::Color(Color::MAGENTA),
        Color::MAGENTA.into_fill(), // pra cor entrar junto com kind tem q ser um bundle
    ));
    commands.spawn((
        Position::new(470, 115),
        coin_rect,
        CoinKind::Color(Color::RED),
        Color::RED.into_fill(),
    ));
    commands.spawn((
        Position::new(300, 115),
        coin_rect,
        CoinKind::Jump((PLAYER_VERTICAL_ACCELERATION / 3.0).trunc() as u32),
        Color::CYAN.into_fill(),
    ));
}
