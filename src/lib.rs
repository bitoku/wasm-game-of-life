mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Math;
use web_sys::console;
use fixedbitset::FixedBitSet;
use std::ops::{Deref, DerefMut};

macro_rules! log {
    ( $( $t: tt)* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cells(FixedBitSet);

impl Deref for Cells {
    type Target = FixedBitSet;

    fn deref(&self) -> &FixedBitSet {
        &self.0
    }
}

impl DerefMut for Cells {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Cells,
}

impl Universe {
    fn get_index(&self, row: u32, column:u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                });
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;
        let size = (width * height) as usize;
        let mut cells = Cells(FixedBitSet::with_capacity(size));

        for i in 0..size {
            cells.set(i, Math::random() < 0.3);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = Cells(FixedBitSet::with_capacity((self.height * width) as usize));
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = Cells(FixedBitSet::with_capacity((height * self.width) as usize));
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }
}

impl Universe {
    pub fn get_cells(&self) -> &Cells {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}