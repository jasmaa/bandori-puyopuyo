// board.rs

use std::option::Option;

pub struct Board<T> {
    width: u32,
    height: u32,
    data: Vec<Option<T>>,
}

impl<T> Board<T> {
    
    pub fn new(width: u32, height: u32) -> Board<T> {

        let data = (0..width*height)
            .map(|_| Option::None)
            .collect();

        Board {
            width: width,
            height: height,
            data: data,
        }
    }

}