// enums.rs
// Enums for PuyoPuyo boards

use wasm_bindgen::prelude::*;

// Piece part
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PiecePart {
    Head,
    Tail,
}

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

impl Sprite {
    pub fn get_affiliation(&self) -> Affiliation {
        match self {
            Self::Kasumi | Self::Tae | Self::Saaya | Self::Rimi | Self::Arisa => {
                Affiliation::Popipa
            }
            Self::Ran | Self::Moca | Self::Himari | Self::Tomoe | Self::Tsugumi => {
                Affiliation::Afterglow
            }
            Self::Aya | Self::Hina | Self::Chisato | Self::Maya | Self::Eve => {
                Affiliation::Pasupare
            }
            Self::Yukina | Self::Sayo | Self::Lisa | Self::Ako | Self::Rinko => {
                Affiliation::Roselia
            }
            Self::Kokoro | Self::Kaoru | Self::Hagumi | Self::Kanon | Self::Misaki => {
                Affiliation::Harohapi
            }
        }
    }

    pub fn from_u32(n: u32) -> Self {
        match n {
            0 => Self::Kasumi,
            1 => Self::Tae,
            2 => Self::Saaya,
            3 => Self::Rimi,
            4 => Self::Arisa,
            5 => Self::Ran,
            6 => Self::Moca,
            7 => Self::Himari,
            8 => Self::Tomoe,
            9 => Self::Tsugumi,
            10 => Self::Aya,
            11 => Self::Hina,
            12 => Self::Chisato,
            13 => Self::Maya,
            14 => Self::Eve,
            15 => Self::Yukina,
            16 => Self::Sayo,
            17 => Self::Lisa,
            18 => Self::Ako,
            19 => Self::Rinko,
            20 => Self::Kokoro,
            21 => Self::Kaoru,
            22=> Self::Hagumi,
            23 => Self::Kanon,
            24 => Self::Misaki,
            _ => panic!("Unknown value: {}", n),
        }
    }
}
