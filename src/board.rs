// board.rs

use std::option::Option;

#[derive(Clone)]
pub struct Board<T> {
    width: u32,
    height: u32,
    data: Vec<Option<T>>,
}

impl<T: PartialEq + Copy> Board<T> {
    pub fn new(width: u32, height: u32) -> Board<T> {
        let data = (0..width * height).map(|_| Option::None).collect();

        Board {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (self.width * row + col) as usize
    }

    pub fn floodfill(&mut self, row: u32, col: u32, curr: T) -> u32 {

        let mut count = 0;
        let curr_idx = self.get_index(row, col);
        
        match self.data[curr_idx] {
            Some(v) => {
                if curr == v {
                    self.data[curr_idx] = Option::None;
                    count += 1;
                }
        
                if row as i32 - 1 >= 0 {
                    count += self.floodfill(row - 1, col, v);
                }
                if row + 1 <= self.height {
                    count += self.floodfill(row + 1, col, v);
                }
                if col as i32 - 1 >= 0 {
                    count += self.floodfill(row, col - 1, v);
                }
                if col + 1 <= self.width {
                    count += self.floodfill(row, col + 1, v);
                }

                count
            },
            None => 0,
        }
    }
}
