use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Board = HashMap<(i32, i32), Tile>;
type Instructions = Vec<Instruction>;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Air,
    Wall,
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Move(i32),
    Rot(char),
}

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

fn find_min_x(board: &Board, y: &i32) -> i32 {
    board
        .keys()
        .filter(|(_, b)| b == y)
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .expect("Not empty")
        .0
}
fn find_max_x(board: &Board, y: &i32) -> i32 {
    board
        .keys()
        .filter(|(_, b)| b == y)
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .expect("Not empty")
        .0
}

fn find_min_y(board: &Board, x: &i32) -> i32 {
    board
        .keys()
        .filter(|(a, _)| a == x)
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .expect("Not empty")
        .1
}
fn find_max_y(board: &Board, x: &i32) -> i32 {
    board
        .keys()
        .filter(|(a, _)| a == x)
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .expect("Not empty")
        .1
}

fn next(board: &Board, pos: (i32, i32), dir: i32) -> (i32, i32) {
    match dir {
        0 => (pos.0 + 1, pos.1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 - 1, pos.1),
        _ => (pos.0, pos.1 - 1),
    }
}
// Directions are right: 0, down: 1, left: 2, up: 3
// Start facing to the right (0)
// Returns (x,y,rotation)
fn play_instructions(board: &Board, instrs: &Instructions) -> (i32, i32, i32) {
    //let max_x = board.keys().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap();
    //let max_y = board.keys().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    //println!(
    //    "{:?}",
    //    board
    //        .iter()
    //        .filter(|(k, _)| k.1 == 0)
    //        .collect::<Vec<(&(usize, usize), &Tile)>>()
    //);
    let mut pos = (find_min_x(board, &0), 0);
    let mut dir = 0;
    for instr in instrs.iter() {
        if let Instruction::Move(s) = instr {
            for _ in 0..*s {
                let n = next(board, pos, dir);
                match board.get(&n) {
                    Some(Tile::Air) => pos = n,
                    Some(Tile::Wall) => break,
                    None => {
                        let wrap = match dir {
                            0 => (find_min_x(board, &pos.1), pos.1),
                            1 => (pos.0, find_min_y(board, &pos.0)),
                            2 => (find_max_x(board, &pos.1), pos.1),
                            _ => (pos.0, find_max_y(board, &pos.0)),
                        };
                        match board.get(&wrap) {
                            Some(Tile::Air) => pos = wrap,
                            Some(Tile::Wall) => break,
                            None => panic!("Ran into a non-existing tile on wrap"),
                        }
                    }
                }
            }
        }
        if let Instruction::Rot(c) = instr {
            match c {
                'R' => dir = (dir + 1).rem_euclid(4),
                _ => dir = (dir - 1).rem_euclid(4),
            }
        }
    }
    (pos.0, pos.1, dir)
}

fn main() {
    let (board, instrs) = read_input("input.txt");
    let (x, y, d) = play_instructions(&board, &instrs);
    // I have x and y flipped compared to description
    println!("{}", 1000 * (y + 1) + 4 * (x + 1) + d);
}
