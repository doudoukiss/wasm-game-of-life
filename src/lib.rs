mod utils;

use js_sys::Uint8Array;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

const DEFAULT_WIDTH: u32 = 128;
const DEFAULT_HEIGHT: u32 = 128;
const MAX_DIMENSION: u32 = 512;

const BLOCK: &[(u32, u32)] = &[(0, 0), (0, 1), (1, 0), (1, 1)];
const BLINKER: &[(u32, u32)] = &[(0, 0), (0, 1), (0, 2)];
const GLIDER: &[(u32, u32)] = &[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
const TOAD: &[(u32, u32)] = &[(0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2)];
const BEACON: &[(u32, u32)] = &[
    (0, 0),
    (0, 1),
    (1, 0),
    (1, 1),
    (2, 2),
    (2, 3),
    (3, 2),
    (3, 3),
];
const LWSS: &[(u32, u32)] = &[
    (0, 1),
    (0, 4),
    (1, 0),
    (2, 0),
    (2, 4),
    (3, 0),
    (3, 1),
    (3, 2),
    (3, 3),
];
const MWSS: &[(u32, u32)] = &[
    (0, 2),
    (1, 0),
    (1, 4),
    (2, 5),
    (3, 0),
    (3, 5),
    (4, 1),
    (4, 2),
    (4, 3),
    (4, 4),
    (4, 5),
];
const HWSS: &[(u32, u32)] = &[
    (0, 2),
    (0, 3),
    (1, 0),
    (1, 5),
    (2, 6),
    (3, 0),
    (3, 6),
    (4, 1),
    (4, 2),
    (4, 3),
    (4, 4),
    (4, 5),
    (4, 6),
];
const PULSAR: &[(u32, u32)] = &[
    (0, 2),
    (0, 3),
    (0, 4),
    (0, 8),
    (0, 9),
    (0, 10),
    (2, 0),
    (2, 5),
    (2, 7),
    (2, 12),
    (3, 0),
    (3, 5),
    (3, 7),
    (3, 12),
    (4, 0),
    (4, 5),
    (4, 7),
    (4, 12),
    (5, 2),
    (5, 3),
    (5, 4),
    (5, 8),
    (5, 9),
    (5, 10),
    (7, 2),
    (7, 3),
    (7, 4),
    (7, 8),
    (7, 9),
    (7, 10),
    (8, 0),
    (8, 5),
    (8, 7),
    (8, 12),
    (9, 0),
    (9, 5),
    (9, 7),
    (9, 12),
    (10, 0),
    (10, 5),
    (10, 7),
    (10, 12),
    (12, 2),
    (12, 3),
    (12, 4),
    (12, 8),
    (12, 9),
    (12, 10),
];
const PENTADECATHLON: &[(u32, u32)] = &[
    (0, 1),
    (1, 1),
    (2, 0),
    (2, 2),
    (3, 1),
    (4, 1),
    (5, 1),
    (6, 1),
    (7, 0),
    (7, 2),
    (8, 1),
    (9, 1),
];
const ACORN: &[(u32, u32)] = &[(0, 1), (1, 3), (2, 0), (2, 1), (2, 4), (2, 5), (2, 6)];
const GLIDER_GUN: &[(u32, u32)] = &[
    (5, 1),
    (5, 2),
    (6, 1),
    (6, 2),
    (3, 13),
    (3, 14),
    (4, 12),
    (4, 16),
    (5, 11),
    (5, 17),
    (6, 11),
    (6, 15),
    (6, 17),
    (6, 18),
    (7, 11),
    (7, 17),
    (8, 12),
    (8, 16),
    (9, 13),
    (9, 14),
    (1, 25),
    (2, 23),
    (2, 25),
    (3, 21),
    (3, 22),
    (4, 21),
    (4, 22),
    (5, 21),
    (5, 22),
    (6, 23),
    (6, 25),
    (7, 25),
    (3, 35),
    (3, 36),
    (4, 35),
    (4, 36),
];

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn from_bool(alive: bool) -> Cell {
        if alive {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }

    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    generation: u32,
    last_births: u32,
    last_deaths: u32,
}

impl Universe {
    fn empty(width: u32, height: u32) -> Universe {
        let width = normalize_dimension(width);
        let height = normalize_dimension(height);
        let cells = vec![Cell::Dead; (width * height) as usize];

        Universe {
            width,
            height,
            cells,
            generation: 0,
            last_births: 0,
            last_deaths: 0,
        }
    }

    fn seeded_default() -> Universe {
        let width = DEFAULT_WIDTH;
        let height = DEFAULT_HEIGHT;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
            generation: 0,
            last_births: 0,
            last_deaths: 0,
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn in_bounds(&self, row: u32, column: u32) -> bool {
        row < self.height && column < self.width
    }

    fn reset_stats(&mut self) {
        self.generation = 0;
        self.last_births = 0;
        self.last_deaths = 0;
    }

    fn clear_last_change_counts(&mut self) {
        self.last_births = 0;
        self.last_deaths = 0;
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            if self.in_bounds(row, col) {
                let idx = self.get_index(row, col);
                self.cells[idx] = Cell::Alive;
            }
        }
        self.clear_last_change_counts();
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        Universe::seeded_default()
    }

    pub fn with_size(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();
        Universe::empty(width, height)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn last_births(&self) -> u32 {
        self.last_births
    }

    pub fn last_deaths(&self) -> u32 {
        self.last_deaths
    }

    pub fn alive_count(&self) -> u32 {
        self.cells
            .iter()
            .filter(|cell| **cell == Cell::Alive)
            .count() as u32
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.resize(width, self.height);
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.resize(self.width, height);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let width = normalize_dimension(width);
        let height = normalize_dimension(height);

        self.width = width;
        self.height = height;
        self.cells = vec![Cell::Dead; (width * height) as usize];
        self.reset_stats();
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::Dead);
        self.reset_stats();
    }

    pub fn randomize(&mut self, seed: u32, density_percent: u32) {
        let density = density_percent.min(100);
        let mut state = seed;

        for cell in self.cells.iter_mut() {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let value = ((state >> 16) % 100) as u32;
            *cell = Cell::from_bool(value < density);
        }

        self.reset_stats();
    }

    pub fn seed_pattern(&mut self, name: &str, row: u32, column: u32) -> bool {
        let pattern_name = name.trim().to_ascii_lowercase();
        let pattern = match pattern_name.as_str() {
            "block" => BLOCK,
            "blinker" => BLINKER,
            "toad" => TOAD,
            "beacon" => BEACON,
            "glider" => GLIDER,
            "spaceship" | "lwss" | "lightweight_spaceship" => LWSS,
            "mwss" | "middleweight_spaceship" => MWSS,
            "hwss" | "heavyweight_spaceship" => HWSS,
            "pulsar" => PULSAR,
            "pentadecathlon" => PENTADECATHLON,
            "acorn" => ACORN,
            "glider_gun" | "gosper_glider_gun" => GLIDER_GUN,
            _ => return false,
        };

        for (row_offset, column_offset) in pattern.iter().cloned() {
            let target_row = row.wrapping_add(row_offset) % self.height;
            let target_column = column.wrapping_add(column_offset) % self.width;
            let idx = self.get_index(target_row, target_column);
            self.cells[idx] = Cell::Alive;
        }

        self.clear_last_change_counts();
        true
    }

    // Return a copy of the cell buffer for JavaScript rendering.
    pub fn cells(&self) -> Uint8Array {
        let cells_slice = self.cells.as_slice();
        let u8_slice = unsafe {
            core::slice::from_raw_parts(cells_slice.as_ptr() as *const u8, cells_slice.len())
        };
        Uint8Array::from(u8_slice)
    }

    pub fn set_cell(&mut self, row: u32, column: u32, alive: bool) -> bool {
        if !self.in_bounds(row, column) {
            return false;
        }

        let idx = self.get_index(row, column);
        self.cells[idx] = Cell::from_bool(alive);
        self.clear_last_change_counts();
        true
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        if !self.in_bounds(row, column) {
            return;
        }

        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
        self.clear_last_change_counts();
    }

    pub fn tick_n(&mut self, steps: u32) {
        for _ in 0..steps {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        let mut births = 0;
        let mut deaths = 0;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                if cell == Cell::Dead && next_cell == Cell::Alive {
                    births += 1;
                } else if cell == Cell::Alive && next_cell == Cell::Dead {
                    deaths += 1;
                }

                next[idx] = next_cell;
            }
        }

        self.cells = next;
        self.generation = self.generation.saturating_add(1);
        self.last_births = births;
        self.last_deaths = deaths;
    }
}

fn normalize_dimension(value: u32) -> u32 {
    value.clamp(1, MAX_DIMENSION)
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, Universe};

    fn alive_cells(universe: &Universe) -> Vec<(u32, u32)> {
        let mut cells = Vec::new();
        for row in 0..universe.height() {
            for column in 0..universe.width() {
                let idx = (row * universe.width() + column) as usize;
                if universe.get_cells()[idx] == Cell::Alive {
                    cells.push((row, column));
                }
            }
        }
        cells
    }

    #[test]
    fn stable_block_stays_unchanged() {
        let mut universe = Universe::with_size(6, 6);
        assert!(universe.seed_pattern("block", 2, 2));
        let before = universe.get_cells().to_vec();

        universe.tick_n(10);

        assert_eq!(universe.get_cells(), before.as_slice());
        assert_eq!(universe.generation(), 10);
        assert_eq!(universe.last_births(), 0);
        assert_eq!(universe.last_deaths(), 0);
    }

    #[test]
    fn blinker_returns_after_two_ticks() {
        let mut universe = Universe::with_size(5, 5);
        assert!(universe.seed_pattern("blinker", 2, 1));
        let before = universe.get_cells().to_vec();

        universe.tick_n(2);

        assert_eq!(universe.get_cells(), before.as_slice());
        assert_eq!(universe.generation(), 2);
    }

    #[test]
    fn glider_moves_after_four_ticks() {
        let mut universe = Universe::with_size(8, 8);
        assert!(universe.seed_pattern("glider", 1, 1));

        universe.tick_n(4);

        assert_eq!(
            alive_cells(&universe),
            vec![(2, 3), (3, 4), (4, 2), (4, 3), (4, 4)]
        );
    }

    #[test]
    fn randomize_is_reproducible_for_same_seed() {
        let mut first = Universe::with_size(12, 8);
        let mut second = Universe::with_size(12, 8);
        let mut different = Universe::with_size(12, 8);

        first.randomize(42, 37);
        second.randomize(42, 37);
        different.randomize(43, 37);

        assert_eq!(first.get_cells(), second.get_cells());
        assert_ne!(first.get_cells(), different.get_cells());
    }

    #[test]
    fn seed_pattern_accepts_distinct_named_patterns() {
        let mut universe = Universe::with_size(64, 64);

        assert!(universe.seed_pattern("lwss", 2, 2));
        assert!(universe.seed_pattern("mwss", 12, 2));
        assert!(universe.seed_pattern("hwss", 22, 2));
        assert!(universe.seed_pattern("glider_gun", 32, 10));
        assert!(universe.seed_pattern("acorn", 50, 10));
        assert!(!universe.seed_pattern("unknown", 0, 0));
        assert!(universe.alive_count() > 60);
    }

    #[test]
    fn resize_and_clear_reset_stats() {
        let mut universe = Universe::with_size(10, 10);
        universe.randomize(7, 55);
        universe.tick();

        universe.resize(5, 4);

        assert_eq!(universe.width(), 5);
        assert_eq!(universe.height(), 4);
        assert_eq!(universe.get_cells().len(), 20);
        assert_eq!(universe.alive_count(), 0);
        assert_eq!(universe.generation(), 0);
        assert_eq!(universe.last_births(), 0);
        assert_eq!(universe.last_deaths(), 0);

        assert!(universe.set_cell(2, 2, true));
        universe.tick();
        universe.clear();

        assert_eq!(universe.alive_count(), 0);
        assert_eq!(universe.generation(), 0);
        assert_eq!(universe.last_births(), 0);
        assert_eq!(universe.last_deaths(), 0);
    }
}
