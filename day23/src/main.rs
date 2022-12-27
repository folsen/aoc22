use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};

type Board = HashSet<(i32, i32)>;

#[allow(dead_code)]
fn pretty_print(board: &Board) {
    let (min_x, min_y, max_x, max_y) = bounds(board);
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if board.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print! {"\n"};
    }
}

fn bounds(board: &Board) -> (i32, i32, i32, i32) {
    board
        .iter()
        .fold((1, 1, 0, 0), |(min_x, min_y, max_x, max_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
        })
}

fn read_input(file_name: &str) -> Board {
    let mut board = HashSet::new();
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.expect("line exists").chars().enumerate() {
            if c == '#' {
                board.insert((x as i32, y as i32));
            }
        }
    }
    board
}
fn add_pos(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}
// Returns how many elves are around in the 8 positions
fn look_around(board: &Board, pos: &(i32, i32)) -> i32 {
    let mut ns = 0;
    for n in [
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ] {
        if board.contains(&add_pos(pos, &n)) {
            ns += 1;
        }
    }
    ns
}

fn neighbors_for_dir(dir: &char) -> Vec<(i32, i32)> {
    match dir {
        'N' => vec![(-1, -1), (0, -1), (1, -1)],
        'S' => vec![(1, 1), (0, 1), (-1, 1)],
        'W' => vec![(-1, -1), (-1, 0), (-1, 1)],
        'E' => vec![(1, -1), (1, 0), (1, 1)],
        _ => panic!("Unexpected direction"),
    }
}
fn neighbor(dir: &char) -> (i32, i32) {
    match dir {
        'N' => (0, -1),
        'S' => (0, 1),
        'W' => (-1, 0),
        'E' => (1, 0),
        _ => panic!("Unexpected direction"),
    }
}

fn increment<K>(m: &mut HashMap<K, i32>, k: &K)
where
    K: Eq + Hash + Clone,
{
    match m.get(k) {
        Some(v) => m.insert(k.clone(), v + 1),
        None => m.insert(k.clone(), 1),
    };
}

fn is_free_dir(board: &Board, pos: &(i32, i32), dir: &char) -> bool {
    neighbors_for_dir(dir)
        .iter()
        .map(|n| !board.contains(&add_pos(pos, n)))
        .all(|n| n)
}

fn spread_out(board: &mut Board, until_done: bool) -> i32 {
    let mut look_directions = VecDeque::from(['N', 'S', 'W', 'E']);
    let rounds = if until_done { 9999 } else { 10 };
    for round in 0..rounds {
        let mut proposals: HashMap<(i32, i32), i32> = HashMap::new();
        for elf in board.iter() {
            let alone = look_around(board, elf);
            for d in look_directions.iter() {
                if alone != 0 && is_free_dir(board, elf, d) {
                    let next_pos = add_pos(elf, &neighbor(d));
                    increment(&mut proposals, &next_pos);
                    break;
                }
            }
        }

        let mut new_board = HashSet::new();
        for elf in board.iter() {
            let mut could_move = false;
            if look_around(board, elf) == 0 {
                could_move = true;
                new_board.insert(elf.clone());
            } else {
                for d in look_directions.iter() {
                    let next_pos = add_pos(elf, &neighbor(d));
                    if is_free_dir(board, elf, d) {
                        if proposals[&next_pos] < 2 {
                            new_board.insert(next_pos);
                        } else {
                            new_board.insert(*elf);
                        }
                        could_move = true;
                        break;
                    }
                }
            }
            if !could_move {
                new_board.insert(*elf);
            }
        }
        if until_done && *board == new_board {
            return round;
        }
        *board = new_board;
        // Shuffle the directions
        look_directions.rotate_left(1);
    }

    let mut ans = 0;
    let (min_x, min_y, max_x, max_y) = bounds(board);
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if !board.contains(&(x, y)) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    // Part 1
    let mut board = read_input("input.txt");
    let empty = spread_out(&mut board, false);
    println!("{}", empty);

    // part 2
    let mut board = read_input("input.txt");
    let rounds = spread_out(&mut board, true);
    println!("{}", rounds + 1); // Counting from 0 in Rust
}
