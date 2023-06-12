mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello, {}!", &s));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;
        let cells = (0..(width * height))
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
    fn index(&self, row: usize, column: usize) -> usize {
        (row * self.width) + column
    }

    fn live_neighbour_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for r_delta in [self.height - 1, 0, 1] {
            for c_delta in [self.width - 1, 0, 1] {
                if r_delta == 0 && c_delta == 0 {
                    continue;
                }
                let r = (row + r_delta) % self.height;
                let c = (column + c_delta) % self.width;
                let idx = self.index(r, c);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for r in 0..self.height {
            for c in 0..self.width {
                let idx = self.index(r, c);
                let live_neighbour_count = self.live_neighbour_count(r, c);
                let next_cell = match (self.cells[idx], live_neighbour_count) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, x) if x < 4 => Cell::Alive,
                    (Cell::Alive, x) if x >= 4 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
