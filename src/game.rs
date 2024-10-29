use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::keyboard::{Action, InputController, InputEvent, KeyState};

#[derive(Debug, strum::Display, Clone, Copy, Hash, PartialEq, Eq)]
enum Piece {
    #[strum(to_string = "-")]
    None,
    #[strum(to_string = "R")]
    Red,
    #[strum(to_string = "B")]
    Black,
}

impl Piece {
    fn color(&self) -> Option<Color> {
        match self {
            Piece::None => None,
            Piece::Red => Some(Color::RGB(255, 0, 0)),
            Piece::Black => Some(Color::RGB(0, 0, 0)),
        }
    }
}

impl Distribution<Piece> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Piece {
        match rng.gen_range(0..=2) {
            0 => Piece::None,
            1 => Piece::Red,
            _ => Piece::Black,
        }
    }
}

const ROWS: usize = 5;
const COLS: usize = 5;

const PLAYER_MAX_HORIZONTAL_SPEED: i32 = 15;
const PLAYER_HORIZONTAL_ACELERATION: i32 = 1;

#[derive(Debug, Default)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Default, Debug)]
pub struct MovableEntity {
    position: Coordinates,
    velocity: Coordinates,
}

impl Display for MovableEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "position: ({},{}), velocity: ({},{})",
            self.position.x, self.position.y, self.velocity.x, self.velocity.y
        )?;
        Ok(())
    }
}

trait Enemy {
    fn update(&self);
}

pub struct Game {
    area: Rect,
    pieces: [[Piece; COLS]; ROWS],
    current_player: Piece,
    pieces_dropped: HashMap<Piece, usize>,
    screen_pos: Coordinates,
    player: MovableEntity,
    inputs: InputController,
}

impl Game {
    pub fn new(area: Rect) -> Self {
        let pieces = [[Piece::None; COLS]; ROWS];
        let current_player = Piece::Red;
        let pieces_dropped = HashMap::new();
        let screen_pos = Coordinates::default();
        let player = MovableEntity::default();
        let input_controller = InputController::new();
        Game {
            area,
            pieces,
            current_player,
            pieces_dropped,
            screen_pos,
            player,
            inputs: input_controller,
        }
    }

    pub fn jumble(&mut self) {
        for row in &mut self.pieces {
            for piece in row {
                *piece = rand::random();
            }
        }
    }

    fn cell_sides(&self) -> (i32, i32) {
        let height = self.area.h / ROWS as i32;
        let width = self.area.w / COLS as i32;
        (width, height)
    }

    pub fn handle_click(&mut self, x: usize, y: usize) {
        let row = ROWS * y / self.area.h as usize;
        let col = COLS * x / self.area.w as usize;
        if row > ROWS || col > COLS {
            return; // Sanity check
        }
        if *self.pieces_dropped.get(&self.current_player).unwrap_or(&0) >= 4 {
            return;
        };
        if self.pieces[row][col] != Piece::None {
            return;
        }
        println!("row: {}, col: {}", row, col);
        self.pieces[row][col] = self.current_player;
        self.next_turn();
    }

    pub fn handle_keypress(&mut self, input_event: InputEvent) {
        self.inputs.handle_input_event(input_event);
    }

    fn next_turn(&mut self) {
        self.pieces_dropped
            .entry(self.current_player)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        match self.current_player {
            Piece::None => (),
            Piece::Red => self.current_player = Piece::Black,
            Piece::Black => self.current_player = Piece::Red,
        }
    }

    pub fn update(&mut self) {
        //main game logic here

        if self.inputs.state[Action::Left].is_active() {
            if self.player.velocity.x >= -PLAYER_MAX_HORIZONTAL_SPEED {
                self.player.velocity.x -= PLAYER_HORIZONTAL_ACELERATION;
            }
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
            self.screen_pos.y -= 10
        } else if self.inputs.state[Action::Up].is_active() {
            self.screen_pos.y += 10
        }

        // if self.player.position[1] > 500 {}
        self.player.position.x += self.player.velocity.x;
        self.player.position.y += self.player.velocity.y;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // self.draw_lines(canvas)?;
        // self.draw_pieces(canvas)?;
        self.draw_player(canvas)?;
        Ok(())
    }

    fn draw_player(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLUE);
        let square: Rect = Rect::new(
            self.player.position.x - self.screen_pos.x,
            self.player.position.y - self.screen_pos.y,
            50,
            50,
        );
        println!("draw_player: {}", self.player);
        canvas.fill_rect(square)?;
        Ok(())
    }
    fn draw_pieces(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let (width, height) = self.cell_sides();
        let (width, height) = (width as i16, height as i16);
        for (line, row) in self.pieces.iter().enumerate() {
            for (column, piece) in row.iter().enumerate() {
                let Some(color) = piece.color() else {
                    continue; // skip empty pieces
                };
                canvas.set_draw_color(color);
                let x = (width / 2) + width * column as i16;
                let y = (height / 2) + height * line as i16;
                canvas.filled_circle(x, y, width / 4, color)?;
            }
        }

        Ok(())
    }

    fn draw_lines(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);

        let (cell_width, cell_height) = self.cell_sides();
        for i in 0..5 {
            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(
                    self.area.w - cell_width / 2,
                    cell_height / 2 + i * cell_height,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                Point::new(
                    cell_width / 2 + i * cell_width,
                    self.area.h - cell_height / 2,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
            )?;

            canvas.draw_line(
                Point::new(
                    cell_width / 2 + i * cell_width,
                    self.area.h - cell_height / 2,
                ),
                Point::new(
                    self.area.w - cell_width / 2,
                    cell_height / 2 + i * cell_height,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(
                    self.area.w - (cell_width / 2 + i * cell_width),
                    self.area.h - cell_height / 2,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                Point::new(
                    self.area.w - cell_width / 2,
                    self.area.h - (cell_height / 2 + i * cell_height),
                ),
            )?;
        }
        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.pieces {
            for piece in row {
                write!(f, "{}", piece)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
