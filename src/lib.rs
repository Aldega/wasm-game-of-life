mod utils;

use std::fmt::Display;

use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello  фыва , {}!", name));
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: fixedbitset::FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
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
        let mut next = fixedbitset::FixedBitSet::with_capacity((self.width * self.height) as usize);

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!("cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors);

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) | (false, 3) => true,
                    (true, x) if x > 3 => false,
                    (otherwise, _) => otherwise,
                });
            }
        }

        

        // log!("    it becomes {:?}", next);

        self.cells = next;
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let size = (self.width * self.height) as usize;
        for idx in 0..size {
            if idx != 0 && idx % self.width as usize == 0 { write!(f, "\n")?; }

            let symbol = if self.cells[idx] { '◼' } else { '◻' };
            write!(f, "{symbol}")?;
        }
        Ok(())
    }

}

#[wasm_bindgen]
impl Universe {

    pub fn new() -> Universe {
        utils::set_panic_hook(); // хук запускает перехват паники и вывод его в console.error в браузере.
        let width: u32 = 100;
        let height: u32 = 64;

        let size = (width * height) as usize;

        let mut cells = fixedbitset::FixedBitSet::with_capacity(size);

        for idx in 0..size {
            cells.set(idx, false);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn set_random_cells(&mut self) {
        let size = (self.width * self.height) as usize;

        let mut cells = fixedbitset::FixedBitSet::with_capacity(size);

        for idx in 0..size {
            cells.set(idx, js_sys::Math::random() > 0.5);
        }

        self.cells = cells;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;

        let size = (self.width * self.height) as usize;

        let mut cells = fixedbitset::FixedBitSet::with_capacity(size);

        for idx in 0..size {
            cells.set(idx, false);
        }

        self.cells = cells;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;

        let size = (self.width * self.height) as usize;

        let mut cells = fixedbitset::FixedBitSet::with_capacity(size);

        for idx in 0..size {
            cells.set(idx, false);
        }

        self.cells = cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }  
}

impl Universe {

    pub fn get_cells(&self) -> &fixedbitset::FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    
}






