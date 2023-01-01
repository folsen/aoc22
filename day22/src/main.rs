use ndarray::linalg::general_mat_vec_mul;
use ndarray::{arr1, array, Array1, Array2};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Board = HashMap<(i32, i32), Tile>;
type Board3D = HashMap<(i32, i32, i32), Pixel>;
type Instructions = Vec<Instruction>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Air,
    Wall,
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Move(i32),
    Rot(char),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pixel {
    normal: Array1<i32>,
    right: Array1<i32>,
    original: (i32, i32),
    tile: Tile,
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
        if let Instruction::Move(s) = instr {
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
        if let Instruction::Rot(c) = instr {
            match c {
                'R' => dir = (dir + 1).rem_euclid(4),
                _ => dir = (dir - 1).rem_euclid(4),
            }
        }
    }
    (pos.0, pos.1, dir)
}

fn mat_rotate(matrix: &Array2<i32>, vector: &Array1<i32>) -> Array1<i32> {
    let mut res = arr1(&[0, 0, 0]);
    general_mat_vec_mul(1, &matrix, &vector, 0, &mut res);
    res
}

fn rot_mat_for(normal: &(i32, i32, i32), rot: &char) -> Array2<i32> {
    let rot_xr = array![[1, 0, 0], [0, 0, -1], [0, 1, 0]];
    let rot_xl = array![[1, 0, 0], [0, 0, 1], [0, -1, 0]];
    let rot_yr = array![[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
    let rot_yl = array![[0, 0, -1], [0, 1, 0], [1, 0, 0]];
    let rot_zr = array![[0, -1, 0], [1, 0, 0], [0, 0, 1]];
    let rot_zl = array![[0, 1, 0], [-1, 0, 0], [0, 0, 1]];
    match (normal, rot) {
        ((1, 0, 0), 'R') => rot_xr,
        ((-1, 0, 0), 'R') => rot_xl,
        ((1, 0, 0), 'L') => rot_xl,
        ((-1, 0, 0), 'L') => rot_xr,

        ((0, 1, 0), 'R') => rot_yr,
        ((0, -1, 0), 'R') => rot_yl,
        ((0, 1, 0), 'L') => rot_yl,
        ((0, -1, 0), 'L') => rot_yr,

        ((0, 0, 1), 'R') => rot_zr,
        ((0, 0, -1), 'R') => rot_zl,
        ((0, 0, 1), 'L') => rot_zl,
        ((0, 0, -1), 'L') => rot_zr,
        _ => panic!("Unexpected normal, rotation combo"),
    }
}

fn rotate_dir(dir: &Array1<i32>, normal: &(i32, i32, i32), rot: &char) -> Array1<i32> {
    let rot_mat = rot_mat_for(normal, rot);
    mat_rotate(&rot_mat, &dir)
}

fn convert_to_3d(board: &Board) -> Board3D {
    board
        .iter()
        .map(|(p, t)| {
            (
                (p.0, p.1, 0),
                Pixel {
                    normal: arr1(&[0, 0, 1]),
                    right: arr1(&[1, 0, 0]),
                    original: *p,
                    tile: *t,
                },
            )
        })
        .collect()
}

// Folding at X works without considering what direction the plane is facing so long as it's the first thing we do
// Things get more complicated when folding Y because we've already folded in X
fn fold_x_cw(board3d: &mut Board3D, fold_at: i32) {
    let folding_points: Vec<((i32, i32, i32), Pixel)> = board3d
        .iter()
        .filter(|(p, _)| p.0 <= fold_at)
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();
    let rot_yccw = array![[0, 0, -1], [0, 1, 0], [1, 0, 0]];
    for (p, mut pix) in folding_points {
        board3d.remove(&p);
        let v = arr1(&[p.0 - fold_at, p.1, p.2]);
        let new_v = mat_rotate(&rot_yccw, &v);
        pix.normal = mat_rotate(&rot_yccw, &pix.normal);
        pix.right = mat_rotate(&rot_yccw, &pix.right);
        board3d.insert((new_v[0] + fold_at, new_v[1], new_v[2] - 1), pix);
    }
}
fn fold_x_ccw(board3d: &mut Board3D, fold_at: i32) {
    let folding_points: Vec<((i32, i32, i32), Pixel)> = board3d
        .iter()
        .filter(|(p, _)| p.0 >= fold_at)
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();
    let rot_ycw = array![[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
    for (p, mut pix) in folding_points {
        board3d.remove(&p);
        let v = arr1(&[p.0 - fold_at, p.1, p.2]);
        let new_v = mat_rotate(&rot_ycw, &v);
        assert_eq!(new_v[1], p.1);
        pix.normal = mat_rotate(&rot_ycw, &pix.normal);
        pix.right = mat_rotate(&rot_ycw, &pix.right);
        board3d.insert((new_v[0] + fold_at, new_v[1], new_v[2] - 1), pix);
    }
}
// Folds everything above a given y coordinate 90 degrees towards the center of the cube
// Figuring out where the center of the cube is, is kinda tricky
fn fold_y_cw(board3d: &mut Board3D, fold_at: i32) {
    let folding_points: Vec<((i32, i32, i32), Pixel)> = board3d
        .iter()
        .filter(|(p, _)| p.1 <= fold_at)
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();
    let rot_xcw = array![[1, 0, 0], [0, 0, -1], [0, 1, 0]];
    for (p, mut pix) in folding_points {
        board3d.remove(&p);
        let v = arr1(&[p.0, p.1 - fold_at, p.2]);
        let new_v = mat_rotate(&rot_xcw, &v);
        let new_z = new_v[2];
        let new_y = new_v[1] + fold_at;
        pix.normal = mat_rotate(&rot_xcw, &pix.normal);
        pix.right = mat_rotate(&rot_xcw, &pix.right);
        board3d.insert((new_v[0], new_y, new_z - 1), pix);
    }
}
fn fold_y_ccw(board3d: &mut Board3D, fold_at: i32) {
    let folding_points: Vec<((i32, i32, i32), Pixel)> = board3d
        .iter()
        .filter(|(p, _)| p.1 >= fold_at)
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();
    let rot_xccw = array![[1, 0, 0], [0, 0, 1], [0, -1, 0]];
    for (p, mut pix) in folding_points {
        board3d.remove(&p);
        let v = arr1(&[p.0, p.1 - fold_at, p.2]);
        let new_v = mat_rotate(&rot_xccw, &v);
        let new_z = new_v[2];
        let new_y = new_v[1] + fold_at;
        pix.normal = mat_rotate(&rot_xccw, &pix.normal);
        pix.right = mat_rotate(&rot_xccw, &pix.right);
        board3d.insert((new_v[0], new_y, new_z - 1), pix);
    }
}

fn right_for_normal(normal: (i32, i32, i32)) -> (i32, i32, i32) {
    match normal {
        (1, 0, 0) => (0, 0, -1),
        (0, 1, 0) => (1, 0, 0),
        (0, 0, 1) => (1, 0, 0),
        (-1, 0, 0) => (0, 0, 1),
        (0, -1, 0) => (1, 0, 0),
        (0, 0, -1) => (1, 0, 0),
        _ => panic!("Impossible normal"),
    }
}

#[derive(Debug)]
struct Player {
    pos: Array1<i32>,
    dir: Array1<i32>,
}

// Returns true if we moved, false if we didn't
fn wrapping_move(board3d: &Board3D, player: &mut Player) -> bool {
    let next = player.pos.clone() + player.dir.clone();
    let next_k = (next[0], next[1], next[2]);
    match board3d.get(&next_k) {
        Some(&Pixel {
            normal: _,
            original: _,
            right: _,
            tile: Tile::Wall,
        }) => false,
        Some(&Pixel {
            normal: _,
            original: _,
            right: _,
            tile: Tile::Air,
        }) => {
            player.pos = next;
            true
        }
        // If none then we're at the end, need to go in the direction we wanted
        // to +1 in the opposite of the normal direction and rotate our direction accordingly.
        None => {
            let curr = &board3d[&(player.pos[0], player.pos[1], player.pos[2])];
            let next_wrap = next + (-1 * curr.normal.clone());
            let nw = (next_wrap[0], next_wrap[1], next_wrap[2]);
            match board3d.get(&nw) {
                Some(&Pixel {
                    normal: _,
                    original: _,
                    right: _,
                    tile: Tile::Wall,
                }) => false,
                Some(&Pixel {
                    normal: _,
                    original: _,
                    right: _,
                    tile: Tile::Air,
                }) => {
                    player.dir = -1 * curr.normal.clone();
                    player.pos = next_wrap;
                    true
                }
                None => panic!("We somehow ended up nowhere after a wrap"),
            }
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}
fn fold_and_walk(
    board3d: &mut Board3D,
    instrs: &Instructions,
    original_start: &(i32, i32),
) -> (i32, i32, i32) {
    let mut len_x = 0;
    let mut len_y = 0;
    // For some reason keys().max_by() gave random results so did this instead
    for (x, y, _) in board3d.keys() {
        len_x = len_x.max(*x + 1);
        len_y = len_y.max(*y + 1);
    }
    let per_side = (board3d.len() as f32 / 6_f32).sqrt().trunc() as i32;
    //fold_x(board3d, 3);
    //fold_x(board3d, 7);
    let mut folds = VecDeque::new();
    for i in 1..len_x / per_side {
        folds.push_back(Fold::X(i * per_side - 1));
    }
    for i in 1..len_y / per_side {
        folds.push_back(Fold::Y(i * per_side - 1));
    }

    while !folds.is_empty() {
        let f = folds.pop_front().unwrap();
        match f {
            Fold::X(at) => {
                // Try folding clock-wise
                let z_plane_count = board3d
                    .iter()
                    .filter(|(p, _)| p.0 <= at && p.2 == 0)
                    .count() as i32;
                if z_plane_count == per_side * per_side {
                    fold_x_cw(board3d, at);
                    continue;
                }
                // Try folding counter-clock-wise
                let z_plane_count =
                    board3d.iter().filter(|(p, _)| p.0 > at && p.2 == 0).count() as i32;
                if z_plane_count == per_side * per_side {
                    fold_x_ccw(board3d, at + 1);
                    continue;
                }
            }
            Fold::Y(at) => {
                // Try folding counter-clock-wise
                let z_plane_count = board3d
                    .iter()
                    .filter(|(p, _)| p.1 <= at && p.2 == 0)
                    .count() as i32;
                if z_plane_count == per_side * per_side {
                    fold_y_cw(board3d, at);
                    continue;
                }
                // Try folding clock-wise
                let z_plane_count =
                    board3d.iter().filter(|(p, _)| p.1 > at && p.2 == 0).count() as i32;
                if z_plane_count == per_side * per_side {
                    fold_y_ccw(board3d, at + 1);
                    continue;
                }
            }
        }
        folds.push_back(f);
    }
    // Walk around
    let mut player = Player {
        pos: arr1(&[0, 0, 0]),
        dir: arr1(&[0, 0, 0]),
    };
    let mut start_normal = arr1(&[]);
    for (p, pix) in board3d.iter() {
        if pix.original == *original_start {
            player.pos = arr1(&[p.0, p.1, p.2]);
            start_normal = pix.normal.clone();
            break;
        }
    }
    let start_dir = right_for_normal((start_normal[0], start_normal[1], start_normal[2]));
    player.dir = arr1(&[start_dir.0, start_dir.1, start_dir.2]);

    for instr in instrs.iter() {
        match instr {
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    if !wrapping_move(board3d, &mut player) {
                        // If we didn't move we hit a wall and can skip remaining steps
                        break;
                    }
                }
            }
            Instruction::Rot(d) => {
                let n = &board3d[&(player.pos[0], player.pos[1], player.pos[2])].normal;
                player.dir = rotate_dir(&player.dir, &(n[0], n[1], n[2]), d);
            }
        }
    }
    let og = &board3d[&(player.pos[0], player.pos[1], player.pos[2])];
    let og_normal = (og.normal[0], og.normal[1], og.normal[2]);
    let mut d = 0;
    let mut dir = player.dir;
    for i in 0..4 {
        if dir == og.right {
            if i == 0 {
                break;
            }
            d = 4 - i;
            break;
        }
        dir = rotate_dir(&dir, &og_normal, &'R');
    }
    (og.original.0, og.original.1, d)
}

fn part1() -> i32 {
    let (board, instrs) = read_input("input.txt");
    let (x, y, d) = play_instructions(&board, &instrs);
    // For me x is col and y is row, so here y comes before x, also i count from
    // 0 and instructions count from 1
    println!("Part 1 score: {}", 1000 * (y + 1) + 4 * (x + 1) + d);
    1000 * (y + 1) + 4 * (x + 1) + d
}

fn part2() -> i32 {
    let (board, instrs) = read_input("input.txt");
    let min_x = find_min_x(&board, &0);
    let mut board3d = convert_to_3d(&board);
    let (x, y, d) = fold_and_walk(&mut board3d, &instrs, &(min_x, 0));
    println!("Part 2 score: {}", 1000 * (y + 1) + 4 * (x + 1) + d);
    1000 * (y + 1) + 4 * (x + 1) + d
}
fn main() {
    part1();
    part2();
}

#[test]
fn test_part1() {
    assert_eq!(191010, part1())
}

#[test]
fn test_part2() {
    assert_eq!(55364, part2())
}
