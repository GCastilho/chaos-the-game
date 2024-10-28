use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::keyboard::{Action, KeyState};

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

const PLAYER_MAX_VERTICAL_SPEED: i32 = 15;
const PLAYER_VERTICAL_ACELERATION: i32 = 1;

struct ScreenPos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct KeyboardState {
    up: KeyState,
    down: KeyState,
    left: KeyState,
    right: KeyState,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            up: KeyState::Up,
            down: KeyState::Up,
            left: KeyState::Up,
            right: KeyState::Up,
        }
    }
}

#[derive(Default, Debug)]
pub struct MovableEntity {
    position: [i32; 2],
    velocity: [i32; 2],
}

trait Enemy {
    fn update(&self);
}

pub struct Game {
    area: Rect,
    pieces: [[Piece; COLS]; ROWS],
    current_player: Piece,
    pieces_dropped: HashMap<Piece, usize>,
    screen_pos: ScreenPos,
    current_keyboard_state: KeyboardState,
    previous_keyboard_state: KeyboardState,
    player: MovableEntity,
}

impl Game {
    pub fn new(area: Rect) -> Self {
        let pieces = [[Piece::None; COLS]; ROWS];
        let current_player = Piece::Red;
        let pieces_dropped = HashMap::new();
        let screen_pos = ScreenPos { x: 0, y: 0 };
        let current_keyboard_state = KeyboardState::default();
        let previous_keyboard_state = KeyboardState::default();
        let player = MovableEntity::default();
        Game {
            area,
            pieces,
            current_player,
            pieces_dropped,
            screen_pos,
            current_keyboard_state,
            previous_keyboard_state,
            player,
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

    pub fn handle_keypress(&mut self, movement: Action, keystate: KeyState) {
        match movement {
            Action::Up => self.current_keyboard_state.up = keystate,
            Action::Down => self.current_keyboard_state.down = keystate,
            Action::Left => self.current_keyboard_state.left = keystate,
            Action::Right => self.current_keyboard_state.right = keystate,
        }
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

        if KeyState::Down == self.current_keyboard_state.left {
            if self.player.velocity[0] >= -PLAYER_MAX_VERTICAL_SPEED {
                self.player.velocity[0] -= PLAYER_VERTICAL_ACELERATION;
            }
        } else if KeyState::Down == self.current_keyboard_state.right
            && self.player.velocity[0] <= PLAYER_MAX_VERTICAL_SPEED
        {
            self.player.velocity[0] += PLAYER_VERTICAL_ACELERATION;
        }

        if KeyState::Up == self.current_keyboard_state.left
            && KeyState::Down == self.previous_keyboard_state.left
        {
            self.player.velocity[0] = 0;
        } else if KeyState::Up == self.current_keyboard_state.right
            && KeyState::Down == self.previous_keyboard_state.right
        {
            self.player.velocity[0] = 0;
        }

        if let KeyState::Down = self.current_keyboard_state.down {
            self.screen_pos.y -= 10
        } else if let KeyState::Down = self.current_keyboard_state.up {
            self.screen_pos.y += 10
        }

        // if self.player.position[1] > 500 {}
        self.player.position[0] += self.player.velocity[0];
        self.player.position[1] += self.player.velocity[1];

        self.previous_keyboard_state = self.current_keyboard_state;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // self.draw_lines(canvas)?;
        self.draw_pieces(canvas)?;
        self.draw_player(canvas)?;
        Ok(())
    }

    fn draw_player(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLUE);
        let square: Rect = Rect::new(
            self.player.position[0] - self.screen_pos.x,
            self.player.position[1] - self.screen_pos.y,
            50,
            50,
        );
        println!("draw_player: {:?}", self.player);
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
                // canvas.filled_circle(x, y, width / 4, color)?;
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
