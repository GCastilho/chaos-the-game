use super::{
    components::{CoinKind, Componentable, Gravitable, Player, Position, Rectangle, Solid, Velocity},
    player::Jump,
};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::Commands;
use sdl2::pixels::Color;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Startup;

pub fn init_player_system(mut commands: Commands) {
    commands.spawn((
        Player,
        Position::new(250, 400),
        Rectangle::new(50, 50),
        Velocity::default(),
        Color::BLUE.into_component(),
        Solid::all(),
        Jump::default(),
        Gravitable,
    ));

    // TODO: Chão não ser inicializado aqui
    commands.spawn((
        Position::new(100, 100),
        Rectangle::new(400, 10),
        Color::GREEN.into_component(),
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
        Color::MAGENTA.into_component(), // pra cor entrar junto com kind tem q ser um bundle
    ));
    commands.spawn((
        Position::new(470, 115),
        coin_rect,
        CoinKind::Color(Color::RED),
        Color::RED.into_component(),
    ));
    commands.spawn((
        Position::new(300, 115),
        coin_rect,
        CoinKind::Jump(20),
        Color::CYAN.into_component(),
    ));
}
