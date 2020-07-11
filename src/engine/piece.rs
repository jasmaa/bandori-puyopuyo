// piece.rs
// PuyoPuyo piece
use super::enums::Sprite;
use super::enums::Direction;

pub struct Piece {
    pub row: u32,
    pub col: u32,
    pub sprite: Sprite,
    pub direction: Direction,
}

impl Piece {
    pub fn new(row: u32, col: u32, sprite: Sprite) -> Piece {
        Piece {
            row: row,
            col: col,
            sprite: sprite,
            direction: Direction::Down,
        }
    }
}