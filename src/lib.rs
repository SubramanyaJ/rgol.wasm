use wasm_bindgen::prelude::*;
use js_sys::Math;

/// A simple Game of Life universe.
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
    /// Create a new random universe with probability `p` of live cells.
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, p: f64) -> Universe {
        let size = (width * height) as usize;
        let mut cells = vec![0; size];
        for c in cells.iter_mut() {
            *c = if Math::random() < p { 1 } else { 0 };
        }
        Universe { width, height, cells }
    }

    /// Advance the universe by one tick.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.idx(row, col);
                let live = self.neighbors(row, col);
                next[idx] = match (self.cells[idx], live) {
                    (1, x) if x < 2 => 0,
                    (1, 2) | (1, 3) => 1,
                    (1, x) if x > 3 => 0,
                    (0, 3) => 1,
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.cells = next;
    }

    /// Pointer to the cells buffer in wasm memory.
    pub fn cells_ptr(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
}

impl Universe {
    fn idx(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn neighbors(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for dr in [self.height - 1, 0, 1] {
            for dc in [self.width - 1, 0, 1] {
                if dr == 0 && dc == 0 { continue; }
                let r = (row + dr) % self.height;
                let c = (col + dc) % self.width;
                count += self.cells[self.idx(r, c)];
            }
        }
        count
    }
}