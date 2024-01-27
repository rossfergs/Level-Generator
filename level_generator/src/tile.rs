use std::fmt;
use std::fmt::write;
use colored::Colorize;

pub struct Tile {
    character: str
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}