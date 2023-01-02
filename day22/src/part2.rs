use crate::common_types::*;

use ndarray::linalg::general_mat_vec_mul;
use ndarray::{arr1, array, Array1, Array2};
use std::collections::{HashMap, VecDeque};
use std::f64::consts::PI;

type Board3D = HashMap<Array1<i32>, Pixel>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pixel {
    normal: Array1<i32>,
    right: Array1<i32>,
    original: (i32, i32),
    tile: Tile,
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Debug)]
struct Player {
    pos: Array1<i32>,
    dir: Array1<i32>,
}

fn mat_rotate(matrix: &Array2<i32>, vector: &Array1<i32>) -> Array1<i32> {
    let mut res = arr1(&[0, 0, 0]);
    general_mat_vec_mul(1, &matrix, &vector, 0, &mut res);
    res
}

fn rotation_matrix(normal: &Array1<i32>, rot: &char) -> Array2<i32> {
    let theta = match rot {
        'R' => PI / 2.0,
        'L' => -PI / 2.0,
        _ => panic!("Unexpected rotation char"),
    };
    let a = normal[0] as f64 * theta;
    let b = normal[1] as f64 * theta;
    let g = normal[2] as f64 * theta;
    let r_x = array![
        [1, 0, 0],
        [0, a.cos() as i32, -a.sin() as i32],
        [0, a.sin() as i32, a.cos() as i32]
    ];
    let r_y = array![
        [b.cos() as i32, 0, b.sin() as i32],
        [0, 1, 0],
        [-b.sin() as i32, 0, b.cos() as i32]
    ];
    let r_z = array![
        [g.cos() as i32, -g.sin() as i32, 0],
        [g.sin() as i32, g.cos() as i32, 0],
        [0, 0, 1]
    ];
    r_x.dot(&r_y).dot(&r_z)
}

fn rotate_dir(dir: &Array1<i32>, normal: &Array1<i32>, rot: &char) -> Array1<i32> {
    let rot_mat = rotation_matrix(normal, rot);
    mat_rotate(&rot_mat, &dir)
}

fn convert_to_3d(board: &Board) -> Board3D {
    board
        .iter()
        .map(|(p, t)| {
            (
                arr1(&[p.0, p.1, 0]),
                Pixel {
                    normal: array![0, 0, 1],
                    right: array![1, 0, 0],
                    original: *p,
                    tile: *t,
                },
            )
        })
        .collect()
}

fn fold(
    board3d: &mut Board3D,
    rot_mat: Array2<i32>,
    fold_at: &Array1<i32>,
    fold_cond: fn(&Array1<i32>, &Array1<i32>) -> bool,
) {
    let mut to_add = HashMap::new();
    board3d.retain(|p, pix| {
        if fold_cond(p, fold_at) {
            let mut new_pix = pix.clone();
            let new_pos = mat_rotate(&rot_mat, &(p - fold_at));
            new_pix.normal = mat_rotate(&rot_mat, &pix.normal);
            new_pix.right = mat_rotate(&rot_mat, &pix.right);
            to_add.insert(new_pos + fold_at + array![0, 0, -1], new_pix.clone());
            false
        } else {
            true
        }
    });
    board3d.extend(to_add);
}

// Returns true if we moved, false if we didn't
fn wrapping_move(board3d: &Board3D, player: &mut Player) -> bool {
    let next = player.pos.clone() + player.dir.clone();
    match board3d.get(&next).map(|p| p.tile) {
        Some(Tile::Wall) => false,
        Some(Tile::Air) => {
            player.pos = next;
            true
        }
        // If none then we're at the end, need to go in the direction we wanted
        // to +1 in the opposite of the normal direction.
        None => {
            let curr = &board3d[&player.pos];
            let next_wrap = next + (-1 * curr.normal.clone());
            match board3d.get(&next_wrap).map(|p| p.tile) {
                Some(Tile::Wall) => false,
                Some(Tile::Air) => {
                    player.dir = -1 * curr.normal.clone();
                    player.pos = next_wrap;
                    true
                }
                None => panic!("We somehow ended up nowhere after a wrap"),
            }
        }
    }
}

fn is_valid_fold(board3d: &Board3D, at: &i32, cond: fn(&Array1<i32>, &i32) -> bool) -> bool {
    let per_side = (board3d.len() as f32 / 6_f32).sqrt().trunc() as i32;
    let z_plane_count = board3d
        .iter()
        .filter(|(p, _)| cond(*p, at) && p[2] == 0)
        .count() as i32;
    z_plane_count == per_side * per_side
}

fn fold_and_walk(
    board3d: &mut Board3D,
    instrs: &Instructions,
    original_start: &(i32, i32),
) -> (i32, i32, i32) {
    let mut len_x = 0;
    let mut len_y = 0;
    // For some reason keys().max_by() gave random results so did this instead
    for p in board3d.keys() {
        len_x = len_x.max(p[0] + 1);
        len_y = len_y.max(p[1] + 1);
    }
    let per_side = (board3d.len() as f32 / 6_f32).sqrt().trunc() as i32;

    // Pre-build a list of all the folding "joints" we have
    let mut folds = VecDeque::new();
    for i in 1..len_x / per_side {
        folds.push_back(Fold::X(i * per_side - 1));
    }
    for i in 1..len_y / per_side {
        folds.push_back(Fold::Y(i * per_side - 1));
    }

    // Try all of them, we might have to skip some but eventually the whole cube will be folded
    // Each fold has a clock-wise and a counter-clockwise variant.
    while !folds.is_empty() {
        let f = folds.pop_front().unwrap();
        match f {
            Fold::X(at) => {
                // Try folding counter-clockwise
                if is_valid_fold(board3d, &at, |p, a| p[0] <= *a) {
                    let rot_yccw = rotation_matrix(&array![0, 1, 0], &'L');
                    fold(board3d, rot_yccw, &array![at, 0, 0], |p, f| p[0] <= f[0]);
                    continue;
                }
                // Try folding clockwise
                if is_valid_fold(board3d, &at, |p, a| p[0] > *a) {
                    let rot_ycw = rotation_matrix(&array![0, 1, 0], &'R');
                    fold(board3d, rot_ycw, &array![at + 1, 0, 0], |p, f| p[0] >= f[0]);
                    continue;
                }
            }
            Fold::Y(at) => {
                // Try folding clockwise
                if is_valid_fold(board3d, &at, |p, a| p[1] <= *a) {
                    let rot_xcw = rotation_matrix(&array![1, 0, 0], &'R');
                    fold(board3d, rot_xcw, &array![0, at, 0], |p, f| p[1] <= f[1]);
                    continue;
                }
                // Try folding counter-clockwise
                if is_valid_fold(board3d, &at, |p, a| p[1] > *a) {
                    let rot_xccw = rotation_matrix(&array![1, 0, 0], &'L');
                    fold(board3d, rot_xccw, &array![0, at + 1, 0], |p, f| {
                        p[1] >= f[1]
                    });
                    continue;
                }
            }
        }
        folds.push_back(f);
    }
    // Walk around
    let mut player = Player {
        pos: array![0, 0, 0],
        dir: array![0, 0, 0],
    };

    for (p, pix) in board3d.iter() {
        if pix.original == *original_start {
            player.pos = p.clone();
            player.dir = pix.right.clone();
            break;
        }
    }

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
                let n = &board3d[&player.pos].normal;
                player.dir = rotate_dir(&player.dir, &n, d);
            }
        }
    }
    let og = &board3d[&player.pos];
    let mut d = 0;
    let mut dir = og.right.clone();
    for i in 0..4 {
        if dir == player.dir {
            d = i;
            break;
        }
        dir = rotate_dir(&dir, &og.normal, &'R');
    }

    (og.original.0, og.original.1, d)
}

pub fn part2(board: Board, instrs: Instructions) -> i32 {
    let min_x = board
        .keys()
        .filter(|(_, b)| *b == 0)
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .expect("Not empty")
        .0;
    let mut board3d = convert_to_3d(&board);
    let (x, y, d) = fold_and_walk(&mut board3d, &instrs, &(min_x, 0));
    println!("Part 2 score: {}", 1000 * (y + 1) + 4 * (x + 1) + d);
    1000 * (y + 1) + 4 * (x + 1) + d
}
