mod board;
mod utils;
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
    sprite_board: Board<Sprite>,
    affiliation_board: Board<Affiliation>,
    direction_board: Board<Direction>,
}

impl Engine {
    pub fn clear() {

    }
}

#[wasm_bindgen]
impl Engine {
    pub fn new(width: u32, height: u32) -> Engine {
        Engine {
            width: width,
            height: height,
            sprite_board: Board::new(width, height),
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

    pub fn tick(&mut self) {

        self.affiliation_board.floodfill(0, 0, Affiliation::Popipa);
    }
}
