mod common_types;
mod part1;
mod part2;

use common_types::*;
use part1::part1;
use part2::part2;

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input(file_name: &str) -> (Board, Instructions) {
    let mut board: Board = HashMap::new();
    let mut instr: Instructions = Vec::new();
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut parsing_board = true;
    for (y, line) in reader.lines().enumerate() {
        let row = line.expect("Should be able to read line");
        if row == "".to_string() {
            parsing_board = false;
            continue;
        }
        if parsing_board {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '.' => board.insert((x as i32, y as i32), Tile::Air),
                    '#' => board.insert((x as i32, y as i32), Tile::Wall),
                    _ => None,
                };
            }
        } else {
            // Parsing instructions
            let mut acc = Vec::new();
            for (i, c) in row.chars().enumerate() {
                if c.is_numeric() {
                    acc.push(c);
                    if i == row.len() - 1 {
                        let i = acc
                            .iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .expect("is num");
                        instr.push(Instruction::Move(i));
                    }
                } else if c == 'R' || c == 'L' {
                    let i = acc
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .expect("is num");
                    acc = vec![];
                    instr.push(Instruction::Move(i));
                    instr.push(Instruction::Rot(c));
                }
            }
        }
    }
    (board, instr)
}

fn main() {
    let (board, instrs) = read_input("input.txt");
    part1(board, instrs);
    let (board, instrs) = read_input("input.txt");
    part2(board, instrs);
}

#[test]
fn test_part1() {
    let (board, instrs) = read_input("input.txt");
    assert_eq!(191010, part1(board, instrs));
    let (board, instrs) = read_input("sample.txt");
    assert_eq!(6032, part1(board, instrs));
}

#[test]
fn test_part2() {
    let (board, instrs) = read_input("input.txt");
    assert_eq!(55364, part2(board, instrs));
    let (board, instrs) = read_input("sample.txt");
    assert_eq!(5031, part2(board, instrs));
}
