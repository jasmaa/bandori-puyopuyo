mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Affiliation {
    Popipa,
    Afterglow,
    Pasupare,
    Roselia,
    Harohapi,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprite {
    Kasumi,
    Tae,
    Rimi,
    Saaya,
    Arisa,
    Ran,
    Moca,
    Himari,
    Tomoe,
    Tsugumi,
    Aya,
    Hina,
    Chisato,
    Maya,
    Eve,
    Yukina,
    Sayo,
    Lisa,
    Ako,
    Rinko,
    Kokoro,
    Kaoru,
    Hagumi,
    Kanon,
    Misaki,
}

#[wasm_bindgen]
pub struct Engine {
    width: u32,
    height: u32,
    sprite_data: Vec<Option<Sprite>>,
    affiliation_data: Vec<Option<Affiliation>>,
    direction_data: Vec<Option<Direction>>,
}

impl Engine {
    // Calculates index from grid row and column
    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (self.width * row + col) as usize
    }

    // Counts blob with floodfill
    pub fn count_blob(&self, row: u32, col: u32) -> u32 {
        let mut affiliation_data_clone = self.affiliation_data.clone();
        let curr_idx = self.get_index(row, col);
        match self.affiliation_data[curr_idx] {
            Some(v) => self.count_blob_aux(&mut affiliation_data_clone, row, col, v),
            None => 0,
        }
    }
    fn count_blob_aux(&self, data: &mut Vec<Option<Affiliation>>, row: u32, col: u32, curr: Affiliation) -> u32 {
        let mut count = 0;
        let curr_idx = self.get_index(row, col);
        match self.affiliation_data[curr_idx] {
            Some(v) => {
                if curr == v {
                    data[curr_idx] = Option::None;
                }

                // Floodfill
                if row as i32 - 1 >= 0 {
                    count += self.count_blob_aux(data, row - 1, col, v);
                }
                if row + 1 <= self.height {
                    count += self.count_blob_aux(data, row + 1, col, v);
                }
                if col as i32 - 1 >= 0 {
                    count += self.count_blob_aux(data, row, col - 1, v);
                }
                if col + 1 <= self.width {
                    count += self.count_blob_aux(data, row, col + 1, v);
                }

                count
            }
            None => 0,
        }
    }

    // Clears blob with floodfill
    pub fn clear_blob(&mut self, row: u32, col: u32) {
        let curr_idx = self.get_index(row, col);
        match self.affiliation_data[curr_idx] {
            Some(v) => self.clear_blob_aux(row, col, v),
            None => (),
        }
    }
    fn clear_blob_aux(&mut self, row: u32, col: u32, curr: Affiliation) {
        let curr_idx = self.get_index(row, col);
        match self.affiliation_data[curr_idx] {
            Some(v) => {
                if curr == v {
                    self.affiliation_data[curr_idx] = Option::None;
                    self.sprite_data[curr_idx] = Option::None;
                    self.direction_data[curr_idx] = Option::None;
                }

                // Floodfill
                if row as i32 - 1 >= 0 {
                    self.clear_blob_aux(row - 1, col, v);
                }
                if row + 1 <= self.height {
                    self.clear_blob_aux(row + 1, col, v);
                }
                if col as i32 - 1 >= 0 {
                    self.clear_blob_aux(row, col - 1, v);
                }
                if col + 1 <= self.width {
                    self.clear_blob_aux(row, col + 1, v);
                }
            }
            None => (),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn new(width: u32, height: u32) -> Engine {
        Engine {
            width: width,
            height: height,
            sprite_data: (0..width * height).map(|_| Option::None).collect(),
            affiliation_data: (0..width * height).map(|_| Option::None).collect(),
            direction_data: (0..width * height).map(|_| Option::None).collect(),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn tick(&mut self) {}
}
