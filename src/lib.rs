mod utils;

use rand::Rng;
use std::{fmt, ops};
use wasm_bindgen::prelude::*;
use web_sys::console as web_console;

use fixedbitset::FixedBitSet;

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: FixedBitSet,
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug)]
pub enum FillOptions {
    AllDead,
    AllAlive,
    Random,
    i2i7_Pattern,
}

impl fmt::Display for FillOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        web_console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        web_console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize, fill_option: FillOptions) -> Universe {
        let size = width * height;
        let cells = FixedBitSet::with_capacity(size);

        let bytes = (size as f64 / 8_f64).ceil() as usize;
        log!(
            "Created universe of size {}x{} and filling options is: {}, total memory: {} bits ({}B)",
            width,
            height,
            fill_option,
            size,
            bytes
        );

        let mut universe = Universe {
            width,
            height,
            cells,
        };
        universe.fill(fill_option);

        universe
    }

    pub fn fill(&mut self, fill_option: FillOptions) {
        let size = self.width * self.height;
        match fill_option {
            FillOptions::AllDead => {
                for i in 0..size {
                    self.cells.set(i, false);
                }
            }
            FillOptions::AllAlive => {
                for i in 0..size {
                    self.cells.set(i, true);
                }
            }
            FillOptions::Random => {
                for i in 0..size {
                    let alive = rand::thread_rng().gen_range(0..100) > 40;
                    self.cells.set(i, alive);
                }
            }
            FillOptions::i2i7_Pattern => {
                for i in 0..size {
                    self.cells.set(i, i % 2 == 0 || i % 7 == 0);
                }
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        (row % self.height) * self.width + (column % self.width)
    }

    pub fn live_neighbours_count(&self, row: usize, column: usize) -> usize {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };
    
        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };
    
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
    
        count as usize
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let idx = self.get_index(row, col);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let current = self.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = current.cells[idx];
                let live_neighbours_count = current.live_neighbours_count(row, column);

                // log!(
                //     "cell [{}, {}] is {} and has {} alive neighbours",
                //     row,
                //     column,
                //     if cell { "Alive" } else { "Dead" },
                //     live_neighbours_count
                // );
                self.cells.set(
                    idx,
                    match (cell, live_neighbours_count) {
                        (true, x) if x < 2 => false,
                        (true, 2..=3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
                // log!(
                //     "   it becomes {}",
                //     if self.cells[idx] { "Alive" } else { "Dead" }
                // );
            }
        }
    }

    pub fn set_cells(&mut self, state: bool, xs: &[usize], ys: &[usize]) {
        log!(
            "r: xs: {:#?}
r: ys: {:#?}",
            &xs,
            &ys
        );
        for i in 0..xs.len() {
            let idx = self.get_index(xs[i], ys[i]);
            self.cells.set(idx, state);
        }
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[u32] {
        self.cells.as_slice()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut idx = 0_usize;
        for _ in 0..self.height {
            for _ in 0..self.width {
                let symbol = if self.cells[idx] { '◼' } else { '◻' };
                idx += 1_usize;
                write!(f, "{}", symbol);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

impl ops::Index<(usize, usize)> for Universe {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = self.get_index(index.0, index.1);
        &self.cells[index]
    }
}
