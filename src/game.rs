use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp;
use typed_builder::TypedBuilder;

use crate::keyboard::{Action, InputController, InputEvent};

const PLAYER_MAX_HORIZONTAL_SPEED: i32 = 15;
const PLAYER_HORIZONTAL_ACCELERATION: i32 = 1;

const PLAYER_MAX_VERTICAL_SPEED: i32 = 15;
const PLAYER_VERTICAL_ACCELERATION: i32 = 1;

// TODO: Rename to Entity
trait Registrable {
    fn set_entity_id(&mut self, entity_id: EntityId);
    fn build_entity(&self) -> Entity;
}

struct EntityId(Option<usize>);

impl EntityId {
    pub fn none() -> Self {
        Self(None)
    }
}

#[derive(Debug, Default)]
struct ECS {
    entities: Vec<Entity>,
}

impl ECS {
    fn get_entity(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id.0?)
    }

    fn get_entity_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(id.0?)
    }

    fn register(&mut self, registrable: &mut impl Registrable) {
        let entity = registrable.build_entity();
        let id = self.entities.len();
        self.entities.push(entity);
        registrable.set_entity_id(EntityId(Some(id)));
    }
}

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

enum CoinKind {
    Color(Color),
    Jump(u32),
}

struct Coin {
    entity_id: EntityId,
    kind: CoinKind,
    start_x: i32,
}

impl Coin {
    pub fn handle_collision_with(&self, player: &mut Player) {
        match self.kind {
            CoinKind::Color(color) => player.entity.color = color,
            CoinKind::Jump(amount) => player.velocity.y = amount as i32,
        }
    }
}

impl Registrable for Coin {
    fn set_entity_id(&mut self, entity_id: EntityId) {
        self.entity_id = entity_id
    }

    fn build_entity(&self) -> Entity {
        let color = match self.kind {
            CoinKind::Color(color) => color,
            CoinKind::Jump(_) => Color::CYAN,
        };
        Entity::builder()
            .color(color)
            .y(115)
            .x(self.start_x)
            .height(10)
            .width(10)
            .build()
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
    coins: Vec<Coin>,
    inputs: InputController,
    ecs: ECS,
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

        let mut ecs = ECS::default();
        let mut coins = vec![
            Coin {
                entity_id: EntityId::none(),
                kind: CoinKind::Color(Color::MAGENTA),
                start_x: 120,
            },
            Coin {
                entity_id: EntityId::none(),
                kind: CoinKind::Color(Color::RED),
                start_x: 470,
            },
            Coin {
                entity_id: EntityId::none(),
                kind: CoinKind::Jump(20),
                start_x: 300,
            },
        ];

        for coin in coins.iter_mut() {
            ecs.register(coin)
        }

        Game {
            player: new_player,
            floor,
            coins,
            ecs,
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

        for coin in &self.coins {
            if let Some(entity) = self.ecs.get_entity(&coin.entity_id) {
                if self.player.entity.colides_with(entity) {
                    coin.handle_collision_with(&mut self.player);
                }
            }
        }
    }

    fn gravitate(&mut self) {
        self.player.velocity.y -= PLAYER_VERTICAL_ACCELERATION;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.floor.draw(canvas)?;
        for entity in &self.ecs.entities {
            entity.draw(canvas)?;
        }
        self.player.entity.draw(canvas)?;
        Ok(())
    }
}
