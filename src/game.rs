use crate::ecs::components::{CollisionAxis, Hitbox, Velocity};
use crate::ecs::Entity;
use crate::{
    ecs::{
        components::{CoinKind, Position, Rectangle, Solid},
        Ecs,
    },
    keyboard::{Action, InputController, InputEvent},
};
use itertools::izip;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::{RefCell, RefMut};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Index;

const PLAYER_MAX_HORIZONTAL_SPEED: i32 = 15;
const PLAYER_HORIZONTAL_ACCELERATION: i32 = 1;

const PLAYER_MAX_VERTICAL_SPEED: i32 = 15;
const PLAYER_VERTICAL_ACCELERATION: i32 = 1;

struct Player<'a> {
    position: &'a RefCell<Position>,
    velocity: RefMut<'a, Velocity>,
    color: &'a RefCell<Color>,
    rect: &'a Rectangle,
}

impl<'a> Player<'a> {
    fn load(id: &Entity, ecs: &'a Ecs) -> Player<'a> {
        let position = ecs
            .positions()
            .index(**id)
            .as_ref()
            .expect("Player missing position");
        let velocity = ecs
            .velocities()
            .index(**id)
            .as_ref()
            .expect("Player missing velocity")
            .borrow_mut();
        let color = ecs
            .colors()
            .index(**id)
            .as_ref()
            .expect("Player missing color");
        let rect = ecs
            .rects()
            .index(**id)
            .as_ref()
            .expect("Player missing rect");
        Self {
            position,
            velocity,
            color,
            rect,
        }
    }

    fn hitbox(&self) -> Hitbox {
        self.rect.on_position(&self.position)
    }
}

pub struct Game {
    player_entity: Entity,
    inputs: InputController,
    ecs: Ecs,
}

impl Game {
    pub fn new() -> Self {
        let input_controller = InputController::new();

        let mut ecs = Ecs::new();

        let player_entity = ecs
            .create_entity()
            .with_velocity(Velocity::default())
            .with_position(Position::new(150, 600))
            .with_rect(Rectangle::new(50, 50))
            .with_solids(Solid::all())
            .with_color(Color::BLUE)
            .entity();

        let coin_rect = Rectangle {
            width: 10,
            height: 10,
        };

        ecs.create_entity()
            .with_position(Position { x: 100, y: 100 })
            .with_rect(Rectangle {
                height: 10,
                width: 400,
            })
            .with_color(Color::GREEN)
            .with_solids(Solid::all());

        ecs.create_entity()
            .with_rect(coin_rect)
            .with_position(Position { x: 120, y: 115 })
            .with_coin_kind(CoinKind::Color(Color::MAGENTA));

        ecs.create_entity()
            .with_rect(coin_rect)
            .with_position(Position { x: 470, y: 115 })
            .with_coin_kind(CoinKind::Color(Color::RED));

        ecs.create_entity()
            .with_rect(coin_rect)
            .with_position(Position { x: 300, y: 115 })
            .with_coin_kind(CoinKind::Jump(20));

        Game {
            player_entity,
            ecs,
            inputs: input_controller,
        }
    }

    pub fn handle_keypress(&mut self, input_event: InputEvent) {
        self.inputs.handle_input_event(input_event);
    }

    pub fn update(&mut self) {
        //main game logic here

        self.handle_player_input();
        self.gravitate();
        self.move_positions_by_velocity();
        self.handle_collisions();
        self.handle_coin_collisions();
    }

    fn handle_player_input(&mut self) {
        let mut player = Player::load(&self.player_entity, &self.ecs);

        if self.inputs.state[Action::Left].is_active()
            && player.velocity.x >= -PLAYER_MAX_HORIZONTAL_SPEED
        {
            player.velocity.x -= PLAYER_HORIZONTAL_ACCELERATION;
        } else if self.inputs.state[Action::Right].is_active()
            && player.velocity.x <= PLAYER_MAX_HORIZONTAL_SPEED
        {
            player.velocity.x += PLAYER_HORIZONTAL_ACCELERATION;
        }

        if !self.inputs.state[Action::Left].is_active()
            && !self.inputs.state[Action::Right].is_active()
        {
            match player.velocity.x.cmp(&0) {
                Less => player.velocity.x += PLAYER_HORIZONTAL_ACCELERATION,
                Greater => player.velocity.x -= PLAYER_HORIZONTAL_ACCELERATION,
                Equal => (),
            }
        }

        if self.inputs.state[Action::Down].is_active() {
            player.velocity.y = -10
        } else if self.inputs.state[Action::Up].is_active() {
            player.velocity.y = 10
        }
    }

    fn gravitate(&mut self) {
        for velocity in self
            .ecs
            .velocities()
            .iter()
            .filter_map(|vel| vel.as_ref().filter(|v| v.borrow().gravitable))
        {
            velocity.borrow_mut().y -= PLAYER_VERTICAL_ACCELERATION;
        }
    }

    fn move_positions_by_velocity(&mut self) {
        for (pos, vel) in izip!(self.ecs.positions(), self.ecs.velocities(),)
            .filter_map(|(pos, vel)| pos.as_ref().and_then(|p| vel.as_ref().map(|v| (p, v))))
        {
            let velocity = vel.borrow();
            let mut position = pos.borrow_mut();
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
    fn handle_collisions(&self) {
        for (i, (pos, recs, vel)) in izip!(
            self.ecs.positions(),
            self.ecs.rects(),
            self.ecs.velocities(),
            self.ecs.solids(),
        )
        .filter_map(|(pos, recs, vel, solid)| {
            pos.as_ref().and_then(|p| {
                recs.and_then(|r| {
                    vel.as_ref()
                        .and_then(|v| solid.filter(|s| s.on_any()).map(|_| (p, r, v)))
                })
            })
        })
        .enumerate()
        {
            let mut moving_hitbox = recs.on_position(pos);

            for (pos, rect) in izip!(self.ecs.positions(), self.ecs.rects(), self.ecs.solids())
                .enumerate()
                .filter(|(j, _)| i != *j)
                .filter_map(|(_, (pos, rect, solid))| {
                    solid
                        .filter(|solid| solid.on_any())
                        .and_then(|_| pos.as_ref().and_then(|p| rect.map(|r| (p, r))))
                })
            {
                let other_hitbox = rect.on_position(pos);
                if let Some(axis) = moving_hitbox.colides_with_axis(&other_hitbox) {
                    match axis {
                        CollisionAxis::Up | CollisionAxis::Down => {
                            vel.borrow_mut().y = 0;
                        }
                        CollisionAxis::Left | CollisionAxis::Right => {
                            vel.borrow_mut().x = 0;
                        }
                    }
                    match axis {
                        CollisionAxis::Up => {
                            moving_hitbox.pos.borrow_mut().y =
                                other_hitbox.bottom() - moving_hitbox.rect.height as i32;
                        }
                        CollisionAxis::Down => {
                            moving_hitbox.pos.borrow_mut().y = other_hitbox.top();
                        }
                        CollisionAxis::Left => {
                            moving_hitbox.pos.borrow_mut().x = other_hitbox.right();
                        }
                        CollisionAxis::Right => {
                            moving_hitbox.pos.borrow_mut().x =
                                other_hitbox.left() - moving_hitbox.rect.width as i32;
                        }
                    }
                }
            }
        }
    }

    fn handle_coin_collisions(&self) {
        let mut player = Player::load(&self.player_entity, &self.ecs);

        izip!(
            self.ecs.positions(),
            self.ecs.rects(),
            self.ecs.coin_kinds()
        )
        .filter_map(|(pos, rect, color_kinds)| {
            pos.as_ref()
                .and_then(|p| rect.and_then(|r| color_kinds.as_ref().map(|c| (p, r, c))))
        })
        .for_each(|(pos, rect, color)| {
            let hitbox = rect.on_position(pos);
            if player.hitbox().colides_with(&hitbox) {
                match color {
                    CoinKind::Color(color) => {
                        player.color.replace(*color);
                    }
                    CoinKind::Jump(amount) => player.velocity.y = *amount as i32,
                }
            }
        });
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (pos, rect, color) in izip!(self.ecs.positions(), self.ecs.rects(), self.ecs.colors())
            .rev()
            .filter_map(|(pos, rect, color)| {
                pos.as_ref()
                    .and_then(|p| rect.and_then(|r| color.as_ref().map(|c| (p, r, c.borrow()))))
            })
        {
            canvas.set_draw_color(*color);
            let pos = pos.borrow();
            let square: Rect = Rect::new(
                pos.x,
                canvas.window().size().1 as i32 - pos.y - rect.height as i32,
                rect.width,
                rect.height,
            );
            canvas.fill_rect(square)?;
        }
        Ok(())
    }
}
