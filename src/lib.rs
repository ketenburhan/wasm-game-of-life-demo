mod utils;

use fixedbitset::FixedBitSet;
use js_sys::Math::random;
use wasm_bindgen::prelude::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    pub fn get_cells_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.cells.hash(&mut hasher);
        hasher.finish()
    }
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let index = self.get_index(row, col);
            self.cells.set(index, true);
        }
    }
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;

                let index = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[index] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next_buffer = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];

                let live_neighbors = self.live_neighbor_count(row, col);

                next_buffer.set(
                    index,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2 | 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (keep, _) => keep,
                    },
                );
            }
        }
        self.cells = next_buffer;
    }
    pub fn new(width: u32, height: u32) -> Universe {
        let size = (width * height) as usize;
        let mut uni = Universe::new_blank(width, height);

        for i in 0..size {
            uni.cells.set(i, random() < 0.5);
        }
        uni
    }
    pub fn new_blank(width: u32, height: u32) -> Universe {
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
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

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}
