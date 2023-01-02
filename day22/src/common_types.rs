use std::collections::HashMap;

pub type Instructions = Vec<Instruction>;
pub type Board = HashMap<(i32, i32), Tile>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tile {
    Air,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Instruction {
    Move(i32),
    Rot(char),
}
