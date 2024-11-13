use super::components::{Player, Position, Rectangle, Solid, Velocity};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::Commands;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct Startup;

pub fn init_player_system(mut commands: Commands) {
    commands.spawn((
        Player,
        Position::new(250, 400),
        Rectangle::new(50, 50),
        Velocity::default(),
        Solid::all(),
    ));

    // TODO: Chão não ser inicializado aqui
    commands.spawn((
        Position::new(100, 150),
        Rectangle::new(400, 10),
        Solid::all(),
    ));
}
