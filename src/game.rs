use itertools::izip;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp;
use typed_builder::TypedBuilder;

use crate::{
    ecs::{
        components::{CoinKind, Position, Rectangle},
        Ecs,
    },
    keyboard::{Action, InputController, InputEvent},
};

const PLAYER_MAX_HORIZONTAL_SPEED: i32 = 15;
const PLAYER_HORIZONTAL_ACCELERATION: i32 = 1;

const PLAYER_MAX_VERTICAL_SPEED: i32 = 15;
const PLAYER_VERTICAL_ACCELERATION: i32 = 1;

#[derive(Debug, Clone, Copy, Default, TypedBuilder)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum CollisionAxis {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, TypedBuilder, Clone, Copy)]
struct Entity {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
}

impl Entity {
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        let square: Rect = Rect::new(
            self.x,
            canvas.window().size().1 as i32 - self.y - self.height as i32,
            self.width,
            self.height,
        );
        canvas.fill_rect(square)
    }

    pub fn colides_with(&self, other: &Entity) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.bottom() < other.top()
            && self.top() > other.bottom()
    }

    pub fn ecs_colides_with(&self, pos: &Position, rect: &Rectangle) -> bool {
        // madness
        self.left() < pos.x + rect.width as i32
            && self.right() > pos.x
            && self.bottom() < pos.y + rect.height as i32
            && self.top() > pos.y
    }

    pub fn colides_with_axis(&self, other: &Entity) -> Option<CollisionAxis> {
        if !self.colides_with(other) {
            return None;
        }
        use cmp::Ordering::*;

        let y_up = self.top() - other.bottom();
        let y_down = other.top() - self.bottom();
        let x_right = self.right() - other.left();
        let x_left = other.right() - self.left();

        let (y_axis, y_value) = match y_up.cmp(&y_down) {
            Greater | Equal => (CollisionAxis::Down, y_down),
            Less => (CollisionAxis::Up, y_up),
        };

        let (x_axis, x_value) = match x_left.cmp(&x_right) {
            Greater | Equal => (CollisionAxis::Right, x_right),
            Less => (CollisionAxis::Left, x_left),
        };

        match y_value.cmp(&x_value) {
            Greater => Some(x_axis),
            Less => Some(y_axis),
            Equal => None,
        }
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    pub fn top(&self) -> i32 {
        self.y + self.height as i32
    }

    pub fn bottom(&self) -> i32 {
        self.y
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
            .x(150)
            .y(600)
            .height(50)
            .width(50)
            .color(Color::BLUE)
            .build();
        Self { entity, velocity }
    }
}

pub struct Game {
    floor: Entity,
    player: Player,
    inputs: InputController,
    ecs: Ecs,
}

impl Game {
    pub fn new() -> Self {
        let input_controller = InputController::new();
        let new_player = Player::new();

        let floor = Entity::builder()
            .color(Color::GREEN)
            .y(100)
            .x(100)
            .height(10)
            .width(400)
            .build();

        let mut ecs2 = Ecs::new();

        let coin_rect = Rectangle {
            width: 10,
            height: 10,
        };

        // Temp floor
        ecs2.create_entity()
            .with_position(Position { x: 100, y: 150 })
            .with_rect(Rectangle {
                height: 10,
                width: 400,
            })
            .with_color(Color::CYAN);

        ecs2.create_entity()
            .with_rect(coin_rect)
            .with_position(Position { x: 120, y: 115 })
            .with_coind_kind(CoinKind::Color(Color::MAGENTA));

        ecs2.create_entity()
            .with_rect(coin_rect)
            .with_position(Position { x: 470, y: 115 })
            .with_coind_kind(CoinKind::Color(Color::RED));

        ecs2.create_entity()
            .with_rect(coin_rect)
            .with_position(Position { x: 300, y: 115 })
            .with_coind_kind(CoinKind::Jump(20));

        Game {
            player: new_player,
            floor,
            ecs: ecs2,
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
            self.player.velocity.x -= PLAYER_HORIZONTAL_ACCELERATION;
        } else if self.inputs.state[Action::Right].is_active()
            && self.player.velocity.x <= PLAYER_MAX_HORIZONTAL_SPEED
        {
            self.player.velocity.x += PLAYER_HORIZONTAL_ACCELERATION;
        }

        if !self.inputs.state[Action::Left].is_active()
            && !self.inputs.state[Action::Right].is_active()
        {
            match self.player.velocity.x.cmp(&0) {
                cmp::Ordering::Less => self.player.velocity.x += PLAYER_HORIZONTAL_ACCELERATION,
                cmp::Ordering::Greater => self.player.velocity.x -= PLAYER_HORIZONTAL_ACCELERATION,
                cmp::Ordering::Equal => (),
            }
        }

        if self.inputs.state[Action::Down].is_active() {
            self.player.velocity.y = -10
        } else if self.inputs.state[Action::Up].is_active() {
            self.player.velocity.y = 10
        }

        self.gravitate();

        self.player.entity.x += self.player.velocity.x;
        self.player.entity.y += self.player.velocity.y;

        if let Some(axis) = self.player.entity.colides_with_axis(&self.floor) {
            match axis {
                CollisionAxis::Up | CollisionAxis::Down => {
                    self.player.velocity.y = 0;
                }
                CollisionAxis::Left | CollisionAxis::Right => {
                    self.player.velocity.x = 0;
                }
            }
            match axis {
                CollisionAxis::Up => {
                    self.player.entity.y = self.floor.bottom() - self.player.entity.height as i32;
                }
                CollisionAxis::Down => {
                    self.player.entity.y = self.floor.top();
                }
                CollisionAxis::Left => {
                    self.player.entity.x = self.floor.right();
                }
                CollisionAxis::Right => {
                    self.player.entity.x = self.floor.left() - self.player.entity.width as i32;
                }
            }
        }

        // Service: handle player colision with coin
        izip!(
            self.ecs.positions(),
            self.ecs.rects(),
            self.ecs.coin_kinds()
        )
        .filter_map(|(pos, rect, color_kinds)| {
            pos.and_then(|p| rect.and_then(|r| color_kinds.map(|c| (p, r, c))))
        })
        .for_each(|(pos, rect, color)| {
            if self.player.entity.ecs_colides_with(&pos, &rect) {
                match color {
                    CoinKind::Color(color) => self.player.entity.color = color,
                    CoinKind::Jump(amount) => self.player.velocity.y = amount as i32,
                }
            }
        });
    }

    fn gravitate(&mut self) {
        self.player.velocity.y -= PLAYER_VERTICAL_ACCELERATION;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.floor.draw(canvas)?;

        for (pos, rect, color) in izip!(self.ecs.positions(), self.ecs.rects(), self.ecs.colors())
            .filter_map(|(pos, rect, color)| {
                pos.and_then(|p| rect.and_then(|r| color.map(|c| (p, r, c))))
            })
        {
            canvas.set_draw_color(color);
            let square: Rect = Rect::new(
                pos.x,
                canvas.window().size().1 as i32 - pos.y - rect.height as i32,
                rect.width,
                rect.height,
            );
            canvas.fill_rect(square)?;
        }

        self.player.entity.draw(canvas)?;
        Ok(())
    }
}
