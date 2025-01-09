mod utils;

use wasm_bindgen::prelude::*;

extern crate js_sys;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Filled = 1,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    snake: Snake,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snake {
    direction: Direction,
    body: Vec<u32>,
    alive: bool,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

impl Snake {
    pub fn get_row_col(&self, index: usize, width: u32) -> (u32, u32) {
        let row = (index as u32) / width;
        let col = (index as u32) % width;
        (row, col)
    }
    pub fn get_next_move(&mut self, width: u32) -> (u32, u32) {
        match self.body.last() {
            Some(val) => {
                let (row, col) = self.get_row_col(*val as usize, width);
                match self.direction {
                    Direction::Down => {
                        let x = row - 1;
                        let y = col;
                        (x, y)
                    }
                    Direction::Up => {
                        let x = row + 1;
                        let y = col;
                        (x, y)
                    }
                    Direction::Left => {
                        let x = row;
                        let y = col - 1;
                        (x, y)
                    }
                    Direction::Right => {
                        let x = row;
                        let y = col + 1;
                        (x, y)
                    }
                }
            }
            None => panic!("snake body is empty"),
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 128;
        let height = 128;

        let mut initial_setup: bool = false;

        let cells = (0..width * height)
            .map(|i| {
                if initial_setup {
                    Cell::Empty
                } else {
                    if js_sys::Math::random() < 0.5 {
                        initial_setup = true;
                        Cell::Filled
                    } else {
                        Cell::Empty
                    }
                }
            })
            .collect();

        let snake = Snake {
            direction: Direction::Right,
            body: vec![0],
            alive: true,
        };

        Universe {
            width,
            height,
            cells,
            snake,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {

        log!("    snake status {:?}", self.snake.alive);

        if !self.snake.alive {
            return;
        }
        let mut next = self.cells.clone();

        let snake_next_move = self.snake.get_next_move(self.width);

        if (snake_next_move.0 >= self.width
            || snake_next_move.0 < 0
            || snake_next_move.1 >= self.height
            || snake_next_move.1 < 0)
        {
            self.snake.alive = false;
            return;
        }

        let idx = self.get_index(snake_next_move.0, snake_next_move.1);

        next[idx] = Cell::Filled; // add head to the snake
        let removed_cell = self.snake.body.remove(0); // remove tail
        next[removed_cell as usize] = Cell::Empty; // mark tail as empty
        self.snake.body.push(idx as u32);

        self.cells = next;
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Empty { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
