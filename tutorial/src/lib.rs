mod utils;

use wasm_bindgen::prelude::*;

extern crate js_sys;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

extern crate web_sys;
use web_sys::console;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer <'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

const UNIVERSE_SIZE : u32 = 64;

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = UNIVERSE_SIZE;
        let height = UNIVERSE_SIZE;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells_ptr(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
    }

    pub fn cell_at(&self, row: u32, col: u32) -> Cell {
        if self.cells[self.get_index(row, col)] {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let size = (width * height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    pub fn set_width(&mut self, width: u32) {
        self.resize(width, self.height);
    }

    pub fn set_height(&mut self, height: u32) {
        self.resize(self.width, height);
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;

                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.height {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);


                let next_cell = match (cell, live_neighbours) {
                    (true, 2) | (true, 3) => true,
                    (true, x) if x < 2 || x > 3 => false,
                    (false, 3) => true,
                    (state, _) => state,
                };

                next.set(idx, next_cell);

                if cell != next_cell {
                    log!(
                        "cell[{}, {}] is initially {:?} and has {} live neighbours, it becomes {:?}",
                        row,
                        col,
                        if cell { Cell::Alive } else { Cell::Dead },
                        live_neighbours,
                        if next_cell { Cell::Alive } else { Cell::Dead }

                    );
                }

            }
        }

        self.cells = next;
    }
}

impl Universe {
    pub fn set_cells(&mut self, coords: &[(u32, u32)]) {
        for (row, col) in coords {
            self.cells.insert(self.get_index(*row, *col))
        }
    }

    pub fn get_cells(self) -> Vec<Cell> {
        self.cells
            .as_slice()
            .iter()
            .map(|i| {
                match i {
                    0 => Cell::Dead,
                    _ => Cell::Alive
                }
            })
            .collect()
    }
}