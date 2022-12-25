use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Board = Vec<Vec<Vec<char>>>;

fn read_input(file_name: &str) -> Board {
    let mut board = Vec::new();
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let mut row = Vec::new();
        for (_, c) in line.expect("line exists").chars().enumerate() {
            if c == '.' {
                row.push(vec![]);
            } else {
                row.push(vec![c]);
            }
        }
        board.push(row);
    }
    board
}

fn initialize_new_board(size_x: usize, size_y: usize) -> Board {
    let mut board = Vec::new();
    for _ in 0..size_y {
        let mut row = Vec::new();
        for _ in 0..size_x {
            row.push(Vec::new());
        }
        board.push(row);
    }
    board
}

fn wrap(board: &Board, pos: (usize, usize), step: (i32, i32)) -> (usize, usize) {
    let size_x = board[0].len() as i32;
    let size_y = board.len() as i32;
    let mut next = (pos.0 as i32 + step.0, pos.1 as i32 + step.1);

    if next.0 == 0 {
        next.0 = size_x - 2;
    }
    if next.1 == 0 {
        next.1 = size_y - 2;
    }
    if next.0 == size_x - 1 {
        next.0 = 1;
    }
    if next.1 == size_y - 1 {
        next.1 = 1;
    }
    (next.0 as usize, next.1 as usize)
}

fn step_blizzards(board: &mut Board) {
    let size_x = board[0].len();
    let size_y = board.len();
    let mut new_board = initialize_new_board(size_x, size_y);

    for y in 0..size_y {
        for x in 0..size_x {
            for c in board[y][x].iter() {
                if *c == '#' {
                    new_board[y][x].push(*c);
                    continue;
                }
                let step = match c {
                    '^' => (0, -1),
                    '>' => (1, 0),
                    'v' => (0, 1),
                    '<' => (-1, 0),
                    _ => (0, 0),
                };
                let n = wrap(board, (x, y), step);
                new_board[n.1][n.0].push(*c);
            }
        }
    }
    *board = new_board;
}

#[allow(dead_code)]
fn pretty_print(board: &Board, pos: &(i32, i32)) {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if *pos == (x as i32, y as i32) {
                print!("E");
            } else {
                print!("{}", board[y][x].get(0).unwrap_or(&'.'));
            }
        }
        print! {"\n"};
    }
}

fn simulate(board: &mut Board, start: (i32, i32), goal: (i32, i32), starting_steps: i32) -> i32 {
    // x, y, steps
    let mut queue = VecDeque::from([(start.0, start.1, 0)]);
    let mut fewest_steps = 9999;
    let mut steps = starting_steps;
    let mut seen_cnt = HashMap::new();
    let mut first_seen = HashMap::new();

    while !queue.is_empty() {
        let pos = queue.pop_front().expect("not empty");

        if !first_seen.contains_key(&(pos.0, pos.1)) {
            first_seen.insert((pos.0, pos.1), pos.2);
        } else if first_seen[&(pos.0, pos.1)] == pos.2 {
            continue;
        }

        if !seen_cnt.contains_key(&(pos.0, pos.1)) {
            seen_cnt.insert((pos.0, pos.1), 0);
        } else if seen_cnt[&(pos.0, pos.1)] < 10 {
            seen_cnt.insert((pos.0, pos.1), seen_cnt[&(pos.0, pos.1)] + 1);
        } else {
            // If we're here we've seen the position 3 times or more
            continue;
        }

        if (pos.0, pos.1) == goal {
            fewest_steps = fewest_steps.min(pos.2);
            continue;
        }
        if pos.2 > fewest_steps {
            continue;
        }

        if pos.2 >= steps || steps == 0 {
            steps += 1;
            step_blizzards(board);
        }

        let moves = [(0, 1), (1, 0), (-1, 0), (0, -1)];
        for m in moves.iter() {
            let next = (pos.0 + m.0, pos.1 + m.1);
            if is_valid_move(&board, next) {
                queue.push_back((next.0, next.1, pos.2 + 1));
            }
        }
        if board[pos.1 as usize][pos.0 as usize].is_empty() {
            // if i don't die here, i can wait
            queue.push_back((pos.0, pos.1, pos.2 + 1));
        }
    }
    fewest_steps
}

fn is_valid_move(board: &Board, pos: (i32, i32)) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && pos.0 < board[0].len() as i32
        && pos.1 < board.len() as i32
        && board[pos.1 as usize][pos.0 as usize].is_empty()
}

fn main() {
    let mut board = read_input("input.txt");
    let goal = (board[0].len() as i32 - 2, board.len() as i32 - 1);
    // Part 1
    let steps = simulate(&mut board, (1, 0), goal, 0);
    println!("{:?}", steps);

    // Part 2
    board = read_input("input.txt");
    let steps1 = simulate(&mut board, (1, 0), goal, 0);
    let steps2 = simulate(&mut board, goal, (1, 0), 1);
    let steps3 = simulate(&mut board, (1, 0), goal, 1);
    println!("{:?}", steps1 + steps2 + steps3);
}
