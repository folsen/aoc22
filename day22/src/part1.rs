use crate::common_types::*;

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

fn next(pos: (i32, i32), dir: i32) -> (i32, i32) {
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
    let mut pos = (find_min_x(board, &0), 0);
    let mut dir = 0;
    for instr in instrs.iter() {
        match instr {
            Instruction::Move(s) => {
                for _ in 0..*s {
                    let n = next(pos, dir);
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
            Instruction::Rot(c) => match c {
                'R' => dir = (dir + 1).rem_euclid(4),
                _ => dir = (dir - 1).rem_euclid(4),
            },
        }
    }
    (pos.0, pos.1, dir)
}

pub fn part1(board: Board, instrs: Instructions) -> i32 {
    let (x, y, d) = play_instructions(&board, &instrs);
    // For me x is col and y is row, so here y comes before x, also i count from
    // 0 and instructions count from 1
    println!("Part 1 score: {}", 1000 * (y + 1) + 4 * (x + 1) + d);
    1000 * (y + 1) + 4 * (x + 1) + d
}
