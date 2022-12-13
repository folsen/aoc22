use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Field = Vec<Vec<char>>;
type Pos = (i32, i32);

fn read_input() -> Field {
    let file = File::open("input.txt").expect("Can't read file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            l.expect("Could not parse line")
                .trim()
                .to_string()
                .chars()
                .collect()
        })
        .collect()
}

fn find_char(field: &Field, target: char) -> Pos {
    for (x, row) in field.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if field[x][y] == target {
                return (x as i32, y as i32);
            }
        }
    }
    (0, 0)
}

fn distance(from: Pos, to: Pos) -> i32 {
    (to.0 - from.0).abs() + (to.1 - from.1).abs()
}

fn cmp_option(a: Option<&i32>, b: Option<&i32>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (Some(x), Some(y)) => x.cmp(&y),
    }
}

fn lookup(field: &Field, pos: Pos) -> Option<char> {
    field.get(pos.0 as usize)?.get(pos.1 as usize).copied()
}

fn height_diff(field: &Field, pos1: Pos, pos2: Pos) -> Option<i32> {
    let a = lookup(field, pos1)?;
    let b = lookup(field, pos2)?;
    let r = match (a, b) {
        ('S', y) => y as i32 - 'a' as i32,
        (x, 'E') => 'z' as i32 - x as i32,
        (x, y) => y as i32 - x as i32,
    };
    Some(r)
}

fn cost(field: &Field, a: Pos, b: Pos) -> i32 {
    // If can't find a height (i.e. at edge)
    // Or height is > 1 then set cost to 9999 otherwise 1 for the move
    match height_diff(field, a, b) {
        None => 9999,
        Some(d) => {
            if d > 1 {
                9999
            } else {
                1
            }
        }
    }
}

fn a_star(field: &Field, start: Pos, goal: Pos) -> i32 {
    let mut open_set: HashSet<Pos> = HashSet::new();
    open_set.insert(start);
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g_score: HashMap<Pos, i32> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<Pos, i32> = HashMap::new();
    f_score.insert(start, distance(start, goal));

    while !open_set.is_empty() {
        let o_current = open_set
            .iter()
            .min_by(|a, b| {
                let af = f_score.get(a);
                let bf = f_score.get(b);
                cmp_option(af, bf)
            })
            .cloned();
        if let Some(current) = o_current {
            if current == goal {
                return g_score[&current];
            }
            open_set.remove(&current.clone());
            let possible_steps = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for step in possible_steps {
                let neighbor = (current.0 + step.0, current.1 + step.1);
                let tentative_g_score = g_score[&current] + cost(field, current, neighbor);
                let neighbor_g_score = g_score.get(&neighbor).unwrap_or(&9999);
                if tentative_g_score < *neighbor_g_score {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + distance(neighbor, goal));
                    if !open_set.contains(&neighbor) {
                        open_set.insert(neighbor);
                    }
                }
            }
        }
    }
    0
}

fn main() {
    let field = read_input();
    let start = find_char(&field, 'S');
    let goal = find_char(&field, 'E');

    // Part 1
    let steps = a_star(&field, start, goal);
    println!("Found goal in {} steps", steps);

    // Part 2
    let mut scores: Vec<i32> = Vec::new();
    for (x, row) in field.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if field[x][y] == 'a' {
                scores.push(a_star(&field, (x as i32, y as i32), goal));
            }
        }
    }
    println!("{:?}", scores.iter().filter(|v| **v > 0).min());
}
