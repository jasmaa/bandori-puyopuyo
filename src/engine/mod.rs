// PuyoPuyo engine

use std::fmt;
use wasm_bindgen::prelude::*;

pub mod enums;
use enums::Affiliation;
use enums::Direction;
use enums::Sprite;

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
    fn count_blob_aux(
        &self,
        data: &mut Vec<Option<Affiliation>>,
        row: u32,
        col: u32,
        curr: Affiliation,
    ) -> u32 {
        let mut count = 0;
        let curr_idx = self.get_index(row, col);
        match data[curr_idx] {
            Some(v) => {
                if curr == v {
                    data[curr_idx] = None;
                    count += 1;

                    // Floodfill
                    if row as i32 - 1 >= 0 {
                        count += self.count_blob_aux(data, row - 1, col, v);
                    }
                    if row + 1 < self.height {
                        count += self.count_blob_aux(data, row + 1, col, v);
                    }
                    if col as i32 - 1 >= 0 {
                        count += self.count_blob_aux(data, row, col - 1, v);
                    }
                    if col + 1 < self.width {
                        count += self.count_blob_aux(data, row, col + 1, v);
                    }
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
                    self.affiliation_data[curr_idx] = None;
                    self.sprite_data[curr_idx] = None;
                    self.direction_data[curr_idx] = None;

                    // Floodfill
                    if row as i32 - 1 >= 0 {
                        self.clear_blob_aux(row - 1, col, v);
                    }
                    if row + 1 < self.height {
                        self.clear_blob_aux(row + 1, col, v);
                    }
                    if col as i32 - 1 >= 0 {
                        self.clear_blob_aux(row, col - 1, v);
                    }
                    if col + 1 < self.width {
                        self.clear_blob_aux(row, col + 1, v);
                    }
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
            sprite_data: (0..width * height).map(|_| None).collect(),
            affiliation_data: (0..width * height)
                .map(|i| {
                    if i >= 20 && i < 70 {
                        Some(Affiliation::Popipa)
                    } else {
                        None
                    }
                })
                .collect(),
            direction_data: (0..width * height).map(|_| None).collect(),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {}
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.affiliation_data.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Some(v) => format!("{}", v as i32),
                    None => String::from("-"),
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
