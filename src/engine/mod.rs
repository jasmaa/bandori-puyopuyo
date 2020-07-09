// Engine

use std::fmt;
use wasm_bindgen::prelude::*;

pub mod enums;
use enums::Affiliation;
use enums::Direction;
use enums::Sprite;

// PuyoPuyo piece
pub struct Piece {
    pub row: u32,
    pub col: u32,
    sprite: Sprite,
    affiliation: Affiliation,
    pub direction: Direction,
}

impl Piece {
    pub fn new(row: u32, col: u32, sprite: Sprite, affiliation: Affiliation) -> Piece {
        Piece {
            row: row,
            col: col,
            sprite: sprite,
            affiliation: affiliation,
            direction: Direction::Down,
        }
    }
}

// PuyoPuyo engine
#[wasm_bindgen]
pub struct Engine {
    width: u32,
    height: u32,
    piece: Piece,
    sprite_data: Vec<Option<Sprite>>,
    affiliation_data: Vec<Option<Affiliation>>,
    direction_data: Vec<Option<Direction>>,
}

#[wasm_bindgen]
impl Engine {
    pub fn new(width: u32, height: u32) -> Engine {
        let mut engine = Engine {
            width: width,
            height: height,
            piece: Piece::new(0, width / 2, Sprite::Kasumi, Affiliation::Popipa),
            sprite_data: (0..width * height).map(|_| None).collect(),
            affiliation_data: (0..width * height)
                .map(|i| {
                    if i < 100 {
                        None
                    } else {
                        Some(Affiliation::Harohapi)
                    }
                })
                .collect(),
            direction_data: (0..width * height).map(|_| None).collect(),
        };
        let piece_idx_1 = engine.get_index(engine.piece.row, engine.piece.col);
        let piece_idx_2 = engine.get_index(engine.piece.row + 1, engine.piece.col);
        engine.sprite_data[piece_idx_1] = Some(engine.piece.sprite);
        engine.sprite_data[piece_idx_2] = Some(engine.piece.sprite);
        engine.affiliation_data[piece_idx_1] = Some(engine.piece.affiliation);
        engine.affiliation_data[piece_idx_2] = Some(engine.piece.affiliation);
        engine.direction_data[piece_idx_1] = Some(engine.piece.direction);
        engine.direction_data[piece_idx_2] = Some(engine.piece.direction);
        engine
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

    // Move piece right
    pub fn move_piece_right(&mut self) {
        match self.piece.direction {
            Direction::Up => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row - 1, self.piece.col);
                let right_idx_1 = self.get_index(self.piece.row, self.piece.col + 1);
                let right_idx_2 = self.get_index(self.piece.row - 1, self.piece.col + 1);
                if self.piece.col < self.width - 1
                    && self.affiliation_data[right_idx_1] == None
                    && self.affiliation_data[right_idx_2] == None
                {
                    self.update_board_piece(curr_idx_1, curr_idx_2, right_idx_1, right_idx_2);
                    self.piece.col += 1;
                }
            }
            Direction::Right => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col + 1);
                let right_idx = self.get_index(self.piece.row, self.piece.col + 2);
                if self.piece.col + 1 < self.width - 1 && self.affiliation_data[right_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, curr_idx_2, right_idx);
                    self.piece.col += 1;
                }
            }
            Direction::Down => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row + 1, self.piece.col);
                let right_idx_1 = self.get_index(self.piece.row, self.piece.col + 1);
                let right_idx_2 = self.get_index(self.piece.row + 1, self.piece.col + 1);
                if self.piece.col < self.width - 1
                    && self.affiliation_data[right_idx_1] == None
                    && self.affiliation_data[right_idx_2] == None
                {
                    self.update_board_piece(curr_idx_1, curr_idx_2, right_idx_1, right_idx_2);
                    self.piece.col += 1;
                }
            }
            Direction::Left => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col - 1);
                let right_idx = self.get_index(self.piece.row, self.piece.col + 1);
                if self.piece.col < self.width - 1 && self.affiliation_data[right_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, right_idx, curr_idx_1);
                    self.piece.col += 1;
                }
            }
        }
    }

    // Move piece left
    pub fn move_piece_left(&mut self) {
        match self.piece.direction {
            Direction::Up => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row - 1, self.piece.col);
                let left_idx_1 = self.get_index(self.piece.row, self.piece.col - 1);
                let left_idx_2 = self.get_index(self.piece.row - 1, self.piece.col - 1);
                if self.piece.col > 0
                    && self.affiliation_data[left_idx_1] == None
                    && self.affiliation_data[left_idx_2] == None
                {
                    self.update_board_piece(curr_idx_1, curr_idx_2, left_idx_1, left_idx_2);
                    self.piece.col -= 1;
                }
            }
            Direction::Right => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col + 1);
                let left_idx = self.get_index(self.piece.row, self.piece.col - 1);
                if self.piece.col > 0 && self.affiliation_data[left_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, left_idx, curr_idx_1);
                    self.piece.col -= 1;
                }
            }
            Direction::Down => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row + 1, self.piece.col);
                let left_idx_1 = self.get_index(self.piece.row, self.piece.col - 1);
                let left_idx_2 = self.get_index(self.piece.row + 1, self.piece.col - 1);
                if self.piece.col > 0
                    && self.affiliation_data[left_idx_1] == None
                    && self.affiliation_data[left_idx_2] == None
                {
                    self.update_board_piece(curr_idx_1, curr_idx_2, left_idx_1, left_idx_2);
                    self.piece.col -= 1;
                }
            }
            Direction::Left => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col - 1);
                let left_idx = self.get_index(self.piece.row, self.piece.col - 2);
                if self.piece.col - 1 > 0 && self.affiliation_data[left_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, curr_idx_2, left_idx);
                    self.piece.col -= 1;
                }
            }
        }
    }

    // Move piece down
    pub fn move_piece_down(&mut self) {
        match self.piece.direction {
            Direction::Up => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row - 1, self.piece.col);
                let down_idx = self.get_index(self.piece.row + 1, self.piece.col);
                if self.piece.row < self.height - 1
                    && self.affiliation_data[down_idx] == None
                    && self.affiliation_data[down_idx] == None
                {
                    self.update_board_piece(curr_idx_1, curr_idx_2, down_idx, curr_idx_1);
                    self.piece.row += 1;
                }
            }
            Direction::Right => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col + 1);
                let down_idx_1 = self.get_index(self.piece.row + 1, self.piece.col);
                let down_idx_2 = self.get_index(self.piece.row + 1, self.piece.col + 1);
                if self.piece.row < self.height - 1
                    && self.affiliation_data[down_idx_1] == None
                    && self.affiliation_data[down_idx_2] == None
                {
                    self.update_board_piece(curr_idx_1, curr_idx_2, down_idx_1, down_idx_2);
                    self.piece.row += 1;
                }
            }
            Direction::Down => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row + 1, self.piece.col);
                let down_idx = self.get_index(self.piece.row + 2, self.piece.col);
                if self.piece.row + 1 < self.height - 1 && self.affiliation_data[down_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, curr_idx_2, down_idx);
                    self.piece.row += 1;
                }
            }
            Direction::Left => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col - 1);
                let down_idx_1 = self.get_index(self.piece.row + 1, self.piece.col);
                let down_idx_2 = self.get_index(self.piece.row + 1, self.piece.col - 1);
                if self.piece.row < self.height - 1
                    && self.affiliation_data[down_idx_1] == None
                    && self.affiliation_data[down_idx_2] == None
                {
                    self.update_board_piece(curr_idx_1, curr_idx_2, down_idx_1, down_idx_2);
                    self.piece.row += 1;
                }
            }
        }
    }

    // Checks if piece can be moved down
    pub fn can_move_piece_down(&mut self) -> bool {
        match self.piece.direction {
            Direction::Up => {
                let down_idx = self.get_index(self.piece.row + 1, self.piece.col);
                self.piece.row < self.height - 1
                    && self.affiliation_data[down_idx] == None
                    && self.affiliation_data[down_idx] == None
            }
            Direction::Right => {
                let down_idx_1 = self.get_index(self.piece.row + 1, self.piece.col);
                let down_idx_2 = self.get_index(self.piece.row + 1, self.piece.col + 1);
                self.piece.row < self.height - 1
                    && self.affiliation_data[down_idx_1] == None
                    && self.affiliation_data[down_idx_2] == None
            }
            Direction::Down => {
                let down_idx = self.get_index(self.piece.row + 2, self.piece.col);
                self.piece.row + 1 < self.height - 1 && self.affiliation_data[down_idx] == None
            }
            Direction::Left => {
                let down_idx_1 = self.get_index(self.piece.row + 1, self.piece.col);
                let down_idx_2 = self.get_index(self.piece.row + 1, self.piece.col - 1);
                self.piece.row < self.height - 1
                    && self.affiliation_data[down_idx_1] == None
                    && self.affiliation_data[down_idx_2] == None
            }
        }
    }

    // Rotate piece clockwise
    pub fn rotate_piece(&mut self) {
        match self.piece.direction {
            Direction::Up => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row - 1, self.piece.col);
                let right_idx = self.get_index(self.piece.row, self.piece.col + 1);
                if self.piece.col + 1 < self.width && self.affiliation_data[right_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, curr_idx_1, right_idx);
                    self.piece.direction = Direction::Right;
                }
            }
            Direction::Right => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col + 1);
                let down_idx = self.get_index(self.piece.row + 1, self.piece.col);
                if self.piece.row + 1 < self.height && self.affiliation_data[down_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, curr_idx_1, down_idx);
                    self.piece.direction = Direction::Down;
                }
            }
            Direction::Down => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row + 1, self.piece.col);
                let left_idx = self.get_index(self.piece.row, self.piece.col - 1);
                if self.piece.col as i32 - 1 >= 0 && self.affiliation_data[left_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, curr_idx_1, left_idx);
                    self.piece.direction = Direction::Left;
                }
            }
            Direction::Left => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col - 1);
                let up_idx = self.get_index(self.piece.row - 1, self.piece.col);
                if self.piece.row as i32 - 1 >= 0 && self.affiliation_data[up_idx] == None {
                    self.update_board_piece(curr_idx_1, curr_idx_2, curr_idx_1, up_idx);
                    self.piece.direction = Direction::Up;
                }
            }
        }
    }

    pub fn tick(&mut self) {
        if self.can_move_piece_down() {
            self.move_piece_down();
        } else {
            // TODO: stick, clear, and respawn here
        }
    }
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

    // Update piece on all boards
    fn update_board_piece(
        &mut self,
        curr_idx_1: usize,
        curr_idx_2: usize,
        new_idx_1: usize,
        new_idx_2: usize,
    ) {
        self.sprite_data[curr_idx_1] = None;
        self.sprite_data[curr_idx_2] = None;
        self.affiliation_data[curr_idx_1] = None;
        self.affiliation_data[curr_idx_2] = None;
        self.direction_data[curr_idx_1] = None;
        self.direction_data[curr_idx_2] = None;
        self.sprite_data[new_idx_1] = Some(self.piece.sprite);
        self.sprite_data[new_idx_2] = Some(self.piece.sprite);
        self.affiliation_data[new_idx_1] = Some(self.piece.affiliation);
        self.affiliation_data[new_idx_2] = Some(self.piece.affiliation);
        self.direction_data[new_idx_1] = Some(self.piece.direction);
        self.direction_data[new_idx_2] = Some(self.piece.direction);
    }
}
