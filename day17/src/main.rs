use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Move = (i32, i32);

fn read_input(file_name: String) -> Vec<Move> {
    let file = File::open(file_name).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut moves = Vec::new();
    for b in reader.bytes() {
        match b.expect("Should be able to read the byte") {
            60 => moves.push((-1, 0)),
            62 => moves.push((1, 0)),
            _ => (),
        }
    }
    moves
}

fn move_shape(mov: &(i32, i32), shape: &mut Vec<(i32, i32)>) {
    //println!("move: {:?}", mov);
    for part in shape.iter_mut() {
        part.0 += (*mov).0;
        part.1 += (*mov).1;
    }
}

fn check_mov(mov: &(i32, i32), settled: &HashSet<(i32, i32)>, shape: &Vec<(i32, i32)>) -> bool {
    for c in shape.iter() {
        let next = (mov.0 + c.0, mov.1 + c.1);
        if next.0 < 0 || next.0 > 6 {
            return false;
        }
        if next.1 < 0 {
            return false;
        }
        if settled.contains(&next) {
            return false;
        }
    }
    return true;
}

fn pretty_print(settled: &HashSet<(i32, i32)>, shape: &Vec<(i32, i32)>, top: i32) {
    for y in 0..top + 1 {
        print!("|");
        for x in 0..7 {
            match settled.get(&(x, top - y)) {
                Some(_) => print!("#"),
                None => {
                    let mut occ = false;
                    for c in shape.iter() {
                        if *c == (x, top - y) {
                            occ = true;
                        }
                    }
                    if occ {
                        print!("@");
                    } else {
                        print!(".");
                    }
                }
            }
        }
        println!("|");
    }
}

fn print_pattern(pattern: &str) {
    for (i, c) in pattern.chars().rev().enumerate() {
        if i % 7 == 0 {
            print!("\n");
        }
        print!("{}", c);
    }
}

// Bottom-left available square in the playing field is 0,0
// Then x increases to the right and y increases upwards
// Shapes are defined by the bottom-left coordinate of their bounding box
fn part1(moves: &Vec<Move>) -> (HashSet<(i32, i32)>, i32) {
    let shapes = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let mut settled: HashSet<(i32, i32)> = HashSet::new();
    let mut top_y = 0;
    let mut stones = 0;
    let mut move_cycle = moves.iter().cycle();
    let mut shapes = shapes.iter().cycle();
    while stones < 2022 {
        let mut s = shapes.next().expect("Has to exist").clone();
        move_shape(&(2, top_y + 3), &mut s);
        // Place it in the right spot
        let mut is_settled = false;
        while !is_settled {
            // Check if it needs to settle first before taking a sideways move
            let m = move_cycle.next().expect("It's a cycle, can't be empty");
            // check if it can move in the direction, if so move it
            if check_mov(m, &settled, &s) {
                move_shape(m, &mut s);
            }
            // check if it can move down, if so move it down, otherwise add it to the settled map
            if check_mov(&(0, -1), &settled, &s) {
                move_shape(&(0, -1), &mut s);
            } else {
                for c in s.iter() {
                    if c.1 + 1 > top_y {
                        top_y = c.1 + 1;
                    }
                    settled.insert(*c);
                }
                is_settled = true;
                stones += 1;
            }
        }
    }
    (settled, top_y)
}

// Finds if there is a repeating pattern and returns the height of the repeating
// pattern if there is a repetition.
// This is way too slow.
fn find_repetition(settled: &HashSet<(i32, i32)>, top: i32, stones: i128) -> Option<(i32, i128)> {
    for y in 0..top + 1 {
        let p1: Vec<&(i32, i32)> = settled.iter().filter(|c| c.1 < y).collect();
        let p2: Vec<&(i32, i32)> = settled.iter().filter(|c| c.1 >= y).collect();
        if p1 == p2 {
            return Some((y, stones / 2));
        }
    }
    None
}

fn create_pattern(settled: &HashSet<(i32, i32)>, top: i32) -> String {
    let mut res = "".to_string();
    for y in (top - 40).max(0)..top {
        for x in 0..7 {
            match settled.get(&(x, y)) {
                Some(_) => res.push('#'),
                None => res.push('.'),
            }
        }
    }
    res
}
// In part 2 we have to do something smarter.
// I cheated because I couldn't think of something smart, but the trick is to
// try to find a repeating pattern, calculate the height of that and then just
// multiply that by how many times we can fit that in the trillion rocks.
fn part2(moves: &Vec<Move>) -> (HashSet<(i32, i32)>, i128) {
    let shapes = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let mut settled: HashSet<(i32, i32)> = HashSet::new();
    let mut patterns: HashMap<String, (i128, i128)> = HashMap::new();
    let mut top_y = 0;
    let mut stones: i128 = 0;
    let mut move_cycle = moves.iter().cycle();
    let mut shapes = shapes.iter().cycle();
    let mut all_stones = vec![];
    let big_number = 1_000_000_000_000;
    while stones < big_number {
        let mut s = shapes.next().expect("Has to exist").clone();
        move_shape(&(2, top_y + 3), &mut s);
        // Place it in the right spot
        let mut is_settled = false;
        while !is_settled {
            // Check if it needs to settle first before taking a sideways move
            let m = move_cycle.next().expect("It's a cycle, can't be empty");
            // check if it can move in the direction, if so move it
            if check_mov(m, &settled, &s) {
                move_shape(m, &mut s);
            }
            // check if it can move down, if so move it down, otherwise add it to the settled map
            if check_mov(&(0, -1), &settled, &s) {
                move_shape(&(0, -1), &mut s);
            } else {
                for c in s.iter() {
                    if c.1 + 1 > top_y {
                        top_y = c.1 + 1;
                    }
                    settled.insert(*c);
                }
                all_stones.push(s.clone());
                is_settled = true;
                let pattern = create_pattern(&settled, top_y);
                if patterns.contains_key(&pattern) {
                    let (p_s, p_y) = patterns.get(&pattern).expect("Should contain");
                    let remainder = all_stones
                        .iter()
                        .take((big_number % (stones - *p_s)) as usize)
                        .flatten()
                        .max_by(|a, b| a.1.cmp(&b.1))
                        .expect("Has to be some max")
                        .1;
                    return (
                        settled,
                        (big_number / (stones - *p_s)) * (top_y as i128 - *p_y)
                            + remainder as i128
                            + 1,
                    );
                } else {
                    patterns.insert(pattern, (stones, top_y.into()));
                }
                stones += 1;
            }
        }
    }
    (settled, top_y as i128)
}

fn main() {
    let moves = read_input("input.txt".to_string());

    // Part 1
    let (_finished_board, top) = part1(&moves);
    println!("{:?}", top);

    // Part 2
    let (_finished_board, top) = part2(&moves);
    println!("{:?}", top);
}
