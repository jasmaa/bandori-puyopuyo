// engine.rs
// PuyoPuyo engine

use std::cmp::min;
use std::fmt;
use wasm_bindgen::prelude::*;

pub mod enums;
use enums::Affiliation;
use enums::Direction;
use enums::PiecePart;
use enums::Sprite;

mod piece;
use piece::Piece;

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub struct Engine {
    width: u32,
    height: u32,
    piece: Piece,
    sprite_data: Vec<Option<Sprite>>,
    affiliation_data: Vec<Option<Affiliation>>,
    direction_data: Vec<Option<Direction>>,
    piece_part_data: Vec<Option<PiecePart>>,
    score: u32,
    combo_count: u32,
    is_clearing: bool,
    is_game_over: bool,
}

#[wasm_bindgen]
impl Engine {
    pub fn new(width: u32, height: u32) -> Engine {
        // console_error_panic_hook::set_once();
        let mut engine = Engine {
            width: width,
            height: height,
            piece: Piece::new(0, 0, Sprite::Kasumi),
            sprite_data: (0..width * height).map(|_| None).collect(),
            affiliation_data: (0..width * height).map(|_| None).collect(),
            direction_data: (0..width * height).map(|_| None).collect(),
            piece_part_data: (0..width * height).map(|_| None).collect(),
            score: 0,
            combo_count: 1,
            is_clearing: false,
            is_game_over: false,
        };
        engine.respawn_piece(Sprite::Kasumi);
        engine
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_is_clearing(&self) -> bool {
        self.is_clearing
    }

    pub fn get_is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn get_sprite_data(&self) -> *const Option<Sprite> {
        self.sprite_data.as_ptr()
    }

    pub fn get_direction_data(&self) -> *const Option<Direction> {
        self.direction_data.as_ptr()
    }

    pub fn get_piece_part_data(&self) -> *const Option<PiecePart> {
        self.piece_part_data.as_ptr()
    }

    // Calculates index from grid row and column
    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (self.width * row + col) as usize
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
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(right_idx_1, right_idx_2);
                    self.piece.col += 1;
                }
            }
            Direction::Right => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col + 1);
                let right_idx = self.get_index(self.piece.row, self.piece.col + 2);
                if self.piece.col + 1 < self.width - 1 && self.affiliation_data[right_idx] == None {
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(curr_idx_2, right_idx);
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
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(right_idx_1, right_idx_2);
                    self.piece.col += 1;
                }
            }
            Direction::Left => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col - 1);
                let right_idx = self.get_index(self.piece.row, self.piece.col + 1);
                if self.piece.col < self.width - 1 && self.affiliation_data[right_idx] == None {
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(right_idx, curr_idx_1);
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
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(left_idx_1, left_idx_2);
                    self.piece.col -= 1;
                }
            }
            Direction::Right => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col + 1);
                let left_idx = self.get_index(self.piece.row, self.piece.col - 1);
                if self.piece.col > 0 && self.affiliation_data[left_idx] == None {
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(left_idx, curr_idx_1);
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
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(left_idx_1, left_idx_2);
                    self.piece.col -= 1;
                }
            }
            Direction::Left => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col - 1);
                let left_idx = self.get_index(self.piece.row, self.piece.col - 2);
                if self.piece.col - 1 > 0 && self.affiliation_data[left_idx] == None {
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(curr_idx_2, left_idx);
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
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(down_idx, curr_idx_1);
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
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(down_idx_1, down_idx_2);
                    self.piece.row += 1;
                }
            }
            Direction::Down => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row + 1, self.piece.col);
                let down_idx = self.get_index(self.piece.row + 2, self.piece.col);
                if self.piece.row + 1 < self.height - 1 && self.affiliation_data[down_idx] == None {
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(curr_idx_2, down_idx);
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
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(down_idx_1, down_idx_2);
                    self.piece.row += 1;
                }
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
                    self.piece.direction = Direction::Right;
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(curr_idx_1, right_idx);
                }
            }
            Direction::Right => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col + 1);
                let down_idx = self.get_index(self.piece.row + 1, self.piece.col);
                if self.piece.row + 1 < self.height && self.affiliation_data[down_idx] == None {
                    self.piece.direction = Direction::Down;
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(curr_idx_1, down_idx);
                }
            }
            Direction::Down => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row + 1, self.piece.col);
                let left_idx = self.get_index(self.piece.row, self.piece.col - 1);
                if self.piece.col as i32 - 1 >= 0 && self.affiliation_data[left_idx] == None {
                    self.piece.direction = Direction::Left;
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(curr_idx_1, left_idx);
                }
            }
            Direction::Left => {
                let curr_idx_1 = self.get_index(self.piece.row, self.piece.col);
                let curr_idx_2 = self.get_index(self.piece.row, self.piece.col - 1);
                let up_idx = self.get_index(self.piece.row - 1, self.piece.col);
                if self.piece.row as i32 - 1 >= 0 && self.affiliation_data[up_idx] == None {
                    self.piece.direction = Direction::Up;
                    self.delete_board_piece(curr_idx_1, curr_idx_2);
                    self.place_board_piece(curr_idx_1, up_idx);
                }
            }
        }
    }

    // Update game
    pub fn tick(&mut self) {
        if !self.is_game_over {
            if !self.is_clearing {
                if self.can_move_piece_down() {
                    self.move_piece_down();
                } else {
                    self.is_clearing = true;
                }
            }
            if self.is_clearing {
                // Clear combo
                // TODO: this is still buggy
                for row in 0..self.height {
                    for col in 0..self.width {
                        let blob_count = self.count_blob(row, col);
                        if blob_count >= 10 {
                            self.score += self.combo_count * blob_count;
                            self.combo_count += 1;
                            self.clear_blob(row, col);
                            self.apply_gravity();
                            return;
                        }
                    }
                }
                self.combo_count = 1;
                self.is_clearing = false;

                // Respawn with random sprite
                if self.can_respawn_piece() {
                    let v = (js_sys::Math::random() * 25.0) as u32;
                    self.respawn_piece(Sprite::from_u32(v));
                } else {
                    self.is_game_over = true
                }
            }
        }
    }

    // Reset game
    pub fn reset(&mut self) {
        self.sprite_data.iter_mut().for_each(|e| *e = None);
        self.affiliation_data.iter_mut().for_each(|e| *e = None);
        self.direction_data.iter_mut().for_each(|e| *e = None);
        self.piece_part_data.iter_mut().for_each(|e| *e = None);
        self.score = 0;
        self.is_clearing = false;
        self.is_game_over = false;
        self.respawn_piece(Sprite::Kasumi);
    }
}

impl Engine {
    pub fn can_respawn_piece(&self) -> bool {
        let idx_1 = self.get_index(0, self.width / 2);
        let idx_2 = self.get_index(1, self.width / 2);
        self.affiliation_data[idx_1] == None && self.affiliation_data[idx_2] == None
    }

    // Respawns piece at the top
    pub fn respawn_piece(&mut self, sprite: Sprite) {
        self.piece.row = 0;
        self.piece.col = self.width / 2;
        self.piece.sprite = sprite;
        self.piece.direction = Direction::Down;

        let idx_1 = self.get_index(self.piece.row, self.piece.col);
        let idx_2 = self.get_index(self.piece.row + 1, self.piece.col);
        self.place_board_piece(idx_1, idx_2);
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

    // Place piece on board
    fn place_piece(&mut self, idx_1: usize, idx_2: usize, sprite: Sprite, direction: Direction) {
        self.sprite_data[idx_1] = Some(sprite);
        self.sprite_data[idx_2] = Some(sprite);
        self.affiliation_data[idx_1] = Some(sprite.get_affiliation());
        self.affiliation_data[idx_2] = Some(sprite.get_affiliation());
        self.direction_data[idx_1] = Some(direction);
        self.direction_data[idx_2] = Some(direction);
        self.piece_part_data[idx_1] = Some(PiecePart::Head);
        self.piece_part_data[idx_2] = Some(PiecePart::Tail);
    }

    fn place_board_piece(&mut self, idx_1: usize, idx_2: usize) {
        self.place_piece(idx_1, idx_2, self.piece.sprite, self.piece.direction)
    }

    // Delete piece from board
    fn delete_board_piece(&mut self, idx_1: usize, idx_2: usize) {
        self.sprite_data[idx_1] = None;
        self.sprite_data[idx_2] = None;
        self.affiliation_data[idx_1] = None;
        self.affiliation_data[idx_2] = None;
        self.direction_data[idx_1] = None;
        self.direction_data[idx_2] = None;
        self.piece_part_data[idx_1] = None;
        self.piece_part_data[idx_2] = None;
    }

    // Applies gravity to pieces
    fn apply_gravity(&mut self) {
        for row in (0..self.height - 1).rev() {
            for col in 0..self.width {
                let curr_idx = self.get_index(row, col);
                if self.affiliation_data[curr_idx] != None {
                    match self.direction_data[curr_idx] {
                        Some(Direction::Up) => {
                            match self.piece_part_data[curr_idx] {
                                Some(PiecePart::Head) => {
                                    let up_idx = self.get_index(row - 1, col);
                                    let ground_row = self.find_ground_row(row, col);
                                    let new_idx_1 = self.get_index(ground_row, col);
                                    let new_idx_2 = self.get_index(ground_row - 1, col);
                                    let sprite = self.sprite_data[curr_idx].unwrap();
                                    self.delete_board_piece(curr_idx, up_idx);
                                    self.place_piece(new_idx_1, new_idx_2, sprite, Direction::Up);
                                }
                                Some(PiecePart::Tail) => {}
                                None => panic!("No piece part found"),
                            };
                        }
                        Some(Direction::Right) => {
                            match self.piece_part_data[curr_idx] {
                                Some(PiecePart::Head) => {
                                    let right_idx = self.get_index(row, col + 1);
                                    let ground_row = min(
                                        self.find_ground_row(row, col),
                                        self.find_ground_row(row, col + 1),
                                    );
                                    let new_idx_1 = self.get_index(ground_row, col);
                                    let new_idx_2 = self.get_index(ground_row, col + 1);
                                    let sprite = self.sprite_data[curr_idx].unwrap();
                                    self.delete_board_piece(curr_idx, right_idx);
                                    self.place_piece(
                                        new_idx_1,
                                        new_idx_2,
                                        sprite,
                                        Direction::Right,
                                    );
                                }
                                Some(PiecePart::Tail) => {}
                                None => panic!("No piece part found"),
                            };
                        }
                        Some(Direction::Down) => {
                            match self.piece_part_data[curr_idx] {
                                Some(PiecePart::Head) => {}
                                Some(PiecePart::Tail) => {
                                    let up_idx = self.get_index(row - 1, col);
                                    let ground_row = self.find_ground_row(row, col);
                                    let new_idx_1 = self.get_index(ground_row - 1, col);
                                    let new_idx_2 = self.get_index(ground_row, col);
                                    let sprite = self.sprite_data[curr_idx].unwrap();
                                    self.delete_board_piece(up_idx, curr_idx);
                                    self.place_piece(new_idx_1, new_idx_2, sprite, Direction::Down);
                                }
                                None => panic!("No piece part found"),
                            };
                        }
                        Some(Direction::Left) => {
                            match self.piece_part_data[curr_idx] {
                                Some(PiecePart::Head) => {}
                                Some(PiecePart::Tail) => {
                                    let right_idx = self.get_index(row, col + 1);
                                    let ground_row = min(
                                        self.find_ground_row(row, col),
                                        self.find_ground_row(row, col + 1),
                                    );
                                    let new_idx_1 = self.get_index(ground_row, col + 1);
                                    let new_idx_2 = self.get_index(ground_row, col);
                                    let sprite = self.sprite_data[curr_idx].unwrap();
                                    self.delete_board_piece(right_idx, curr_idx);
                                    self.place_piece(new_idx_1, new_idx_2, sprite, Direction::Left);
                                }
                                None => panic!("No piece part found"),
                            };
                        }
                        None => {}
                    }
                }
            }
        }
    }
    fn find_ground_row(&self, row: u32, col: u32) -> u32 {
        for curr_row in (row + 1)..self.height {
            let curr_idx = self.get_index(curr_row, col);
            if self.affiliation_data[curr_idx] != None {
                return curr_row - 1;
            }
        }
        self.height - 1
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
