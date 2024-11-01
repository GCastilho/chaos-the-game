use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp;
use typed_builder::TypedBuilder;

use crate::keyboard::{Action, InputController, InputEvent};

const PLAYER_MAX_HORIZONTAL_SPEED: i32 = 15;
const PLAYER_HORIZONTAL_ACELERATION: i32 = 1;

const PLAYER_MAX_VERTICAL_SPEED: i32 = 15;
const PLAYER_VERTICAL_ACELERATION: i32 = 1;

#[derive(Debug, Clone, Copy, Default, TypedBuilder)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, TypedBuilder)]
struct Dimentions {
    width: u32,
    height: u32,
}

#[derive(Debug, TypedBuilder, Clone, Copy)]
struct Entity {
    position: Coordinates,
    shape: Dimentions,
    color: Color,
}

impl Entity {
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        let square: Rect = Rect::new(
            self.position.x,
            canvas.window().size().1 as i32 - self.position.y - self.shape.height as i32,
            self.shape.width,
            self.shape.height,
        );
        canvas.fill_rect(square)
    }

    pub fn get_left(&self) -> i32 {
        self.position.x
    }

    pub fn get_right(&self) -> i32 {
        self.position.x + self.shape.width as i32
    }

    pub fn get_up(&self) -> i32 {
        self.position.y + self.shape.height as i32
    }

    pub fn get_down(&self) -> i32 {
        self.position.y
    }
}

struct Player {
    entity: Entity,
    velocity: Coordinates,
}

impl Player {
    pub fn new() -> Self {
        let velocity = Coordinates::default();
        let entity = Entity::builder()
            .position(Coordinates::builder().x(120).y(600).build())
            .shape(Dimentions::builder().height(50).width(50).build())
            .color(Color::BLUE)
            .build();
        Self { entity, velocity }
    }
}

pub struct Game {
    floor: Entity,
    player: Player,
    inputs: InputController,
}

impl Game {
    pub fn new() -> Self {
        let input_controller = InputController::new();
        let new_player = Player::new();

        let floor = Entity::builder()
            .color(Color::GREEN)
            .shape(Dimentions::builder().height(10).width(400).build())
            .position(Coordinates::builder().y(100).x(100).build())
            .build();

        Game {
            player: new_player,
            floor,
            inputs: input_controller,
        }
    }

    pub fn handle_keypress(&mut self, input_event: InputEvent) {
        self.inputs.handle_input_event(input_event);
    }

    fn get_closest_ground(&mut self) -> Entity {
        self.floor
    }

    pub fn update(&mut self) {
        //main game logic here

        if self.inputs.state[Action::Left].is_active()
            && self.player.velocity.x >= -PLAYER_MAX_HORIZONTAL_SPEED
        {
            self.player.velocity.x -= PLAYER_HORIZONTAL_ACELERATION;
        } else if self.inputs.state[Action::Right].is_active()
            && self.player.velocity.x <= PLAYER_MAX_HORIZONTAL_SPEED
        {
            self.player.velocity.x += PLAYER_HORIZONTAL_ACELERATION;
        }

        if !self.inputs.state[Action::Left].is_active()
            && !self.inputs.state[Action::Right].is_active()
        {
            match self.player.velocity.x.cmp(&0) {
                cmp::Ordering::Less => self.player.velocity.x += PLAYER_HORIZONTAL_ACELERATION,
                cmp::Ordering::Greater => self.player.velocity.x -= PLAYER_HORIZONTAL_ACELERATION,
                cmp::Ordering::Equal => (),
            }
        }

        if self.inputs.state[Action::Down].is_active() {
            self.player.velocity.y = -10
        } else if self.inputs.state[Action::Up].is_active() {
            self.player.velocity.y = 10
        }

        self.gravitate();

        self.player.entity.position.x += self.player.velocity.x;
        self.player.entity.position.y += self.player.velocity.y;

        let down_colision = self.get_closest_ground();
        let down_colision_position = down_colision.position.y + down_colision.shape.height as i32;
        println!(
            "down_colision_position: {down_colision_position}; {:?}; {:?}",
            self.player.entity.position, self.player.velocity
        );

        if self.player.velocity.y < 0
            && self.player.entity.get_right() > down_colision.get_left()
            && self.player.entity.get_left() < down_colision.get_right()
            && (self.player.entity.get_down()) < down_colision.get_up()
            && (self.player.entity.get_up()) > down_colision.get_down()
        {
            self.player.entity.position.y = down_colision.get_up();
            self.player.velocity.y = 0;
        }
    }

    fn gravitate(&mut self) {
        if self.player.velocity.y < PLAYER_MAX_VERTICAL_SPEED {
            self.player.velocity.y -= PLAYER_VERTICAL_ACELERATION; // need to be refatored to use a secondary gravity value instead of altering velocity directly
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.floor.draw(canvas)?;
        self.player.entity.draw(canvas)?;
        Ok(())
    }
}
