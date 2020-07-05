mod utils;
mod board;
use board::Board;

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
pub struct Engine {
    width: u32,
    height: u32,
    affiliation_board: Board<Affiliation>,
    direction_board: Board<Direction>,
}

#[wasm_bindgen]
impl Engine {

    pub fn new(width: u32, height: u32) -> Engine {
        Engine {
            width: width,
            height: height,
            affiliation_board: Board::new(width, height),
            direction_board: Board::new(width, height),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}