// enums.rs
// Enums for PuyoPuyo boards

use wasm_bindgen::prelude::*;

// Band affiliation
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

// Piece direction
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

// Piece sprite
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