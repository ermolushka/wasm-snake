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
    target: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snake {
    direction: Direction,
    body: Vec<u32>,
    alive: bool,
}

impl Universe {
    /**
     * To get index as we store the whole universe as 1d array
     */
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

impl Snake {
    /**
     * Inverse of get_index to calculate the next move for the snake
     */
    pub fn get_row_col(&self, index: usize, width: u32) -> (u32, u32) {
        let row = (index as u32) / width;
        let col = (index as u32) % width;
        (row, col)
    }

    /**
     * Get next move's indexes
     */
    pub fn get_next_move(&mut self, width: u32) -> (u32, u32) {
        match self.body.last() {
            Some(val) => {
                let (row, col) = self.get_row_col(*val as usize, width);

                match self.direction {
                    Direction::Up => {
                        if row == 0 {
                            self.alive = false;
                            return (row, col);
                        }
                        (row - 1, col)
                    }
                    Direction::Down => {
                        if row >= width - 1 {
                            // Using width since it's a square grid
                            self.alive = false;
                            return (row, col);
                        }
                        (row + 1, col)
                    }
                    Direction::Left => {
                        if col == 0 {
                            self.alive = false;
                            return (row, col);
                        }
                        (row, col - 1)
                    }
                    Direction::Right => {
                        if col >= width - 1 {
                            self.alive = false;
                            return (row, col);
                        }
                        (row, col + 1)
                    }
                }
            }
            None => panic!("snake body is empty"),
        }
    }
}

/// Public methods, exported to JavaScript using wasm_bindgen
#[wasm_bindgen]
impl Snake {
    #[wasm_bindgen]
    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(getter)]
    pub fn snake(&mut self) -> *mut Snake {
        &mut self.snake as *mut Snake
    }

    /**
     * Change snake's direction based on user's keyboard input
     */
    #[wasm_bindgen]
    pub fn change_snake_direction(&mut self, direction: Direction) {
        let old_direction = self.snake.direction;

        // Don't allow opposite direction changes
        let invalid_change = match (old_direction, direction) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        };

        if !invalid_change {
            self.snake.direction = direction;
        } else {
            log!(
                "Invalid direction change attempted: {:?} to {:?}",
                old_direction,
                direction
            );
        }
    }
    /**
     * Check if snake is alive == game is ongoing
     */
    pub fn snake_is_alive(&self) -> bool {
        self.snake.alive
    }
    pub fn new() -> Universe {
        utils::set_panic_hook();

        // Dimensions hardcoded for now, need to recompile evry time to change it, we can set it from UI but I am lazy.
        // Also, life is too short to learn frontend.

        let width = 32;
        let height = 32;

        // initial random target for the snake
        let random_initial_placement = (js_sys::Math::random() * (width * height) as f64) as u32;

        log!("random initial {:?}", random_initial_placement);

        // Mark every cell either filled or not
        let cells = (0..width * height)
            .map(|i| {
                if i == random_initial_placement {
                    Cell::Filled
                } else {
                    Cell::Empty
                }
            })
            .collect();

        let snake = Snake {
            direction: Direction::Right,
            body: vec![0], // currently, always start at (0, 0), may reconsider in future
            alive: true,
        };

        Universe {
            width,
            height,
            cells,
            snake,
            target: random_initial_placement,
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
        if !self.snake.alive {
            return;
        }

        let mut next = self.cells.clone();
        let snake_next_move = self.snake.get_next_move(self.width);

        // Boundary check
        if snake_next_move.0 >= self.width || snake_next_move.1 >= self.height {
            log!(
                "Snake hit boundary: ({}, {}) vs width={}, height={}",
                snake_next_move.0,
                snake_next_move.1,
                self.width,
                self.height
            );
            self.snake.alive = false;
            return;
        }

        // check snake's next potential movement
        let idx = self.get_index(snake_next_move.0, snake_next_move.1);

        // snake hit itself
        if self.snake.body.contains(&(idx as u32)) {
            self.snake.alive = false;
            return;
        }

        // next move of the snake is to eat the target
        if idx as u32 == self.target {
            next[idx] = Cell::Filled; // add head to the snake
            self.snake.body.push(idx as u32);

            // setting up a new target but it shouldn't be within the snake body, repeat until all good
            loop {
                let new_val = (js_sys::Math::random() * (self.width * self.height) as f64) as u32;
                if !self.snake.body.contains(&new_val) {
                    self.target = new_val;
                    break;
                }
            }
            log!("random after eating {:?}", self.target);

            next[self.target as usize] = Cell::Filled;

        // some repetition in the else statement but I don't care
        } else {
            next[idx] = Cell::Filled; // add head to the snake
            let removed_cell = self.snake.body.remove(0); // remove tail
            next[removed_cell as usize] = Cell::Empty; // mark tail as empty
            self.snake.body.push(idx as u32);
        }

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
