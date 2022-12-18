use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Cube = (i32, i32, i32);

fn read_input(file_name: &str) -> HashSet<Cube> {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut cubes = HashSet::new();
    for line in reader.lines() {
        let coords: Vec<i32> = line
            .expect("Should be able to read line")
            .replace("\n", "")
            .split(",")
            .map(|c| c.parse::<i32>().expect("Parse error"))
            .collect();
        cubes.insert((coords[0], coords[1], coords[2]));
    }
    cubes
}
// A* impl copied from day12
fn a_star(cubes: &HashSet<Cube>, start: Cube) -> bool {
    let mut open_set: HashSet<Cube> = HashSet::new();
    open_set.insert(start);
    let mut came_from: HashMap<Cube, Cube> = HashMap::new();
    let mut g_score: HashMap<Cube, i32> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<Cube, i32> = HashMap::new();
    f_score.insert(start, distance(start, (0, 0, 0)));

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
            if current == (0, 0, 0) {
                return false;
            }
            open_set.remove(&current.clone());
            let possible_steps = [
                (0, 0, 1),
                (0, 1, 0),
                (1, 0, 0),
                (0, 0, -1),
                (0, -1, 0),
                (-1, 0, 0),
            ];
            for step in possible_steps {
                let neighbor = (current.0 + step.0, current.1 + step.1, current.2 + step.2);
                let tentative_g_score = g_score[&current] + 1;
                let neighbor_g_score = g_score.get(&neighbor).unwrap_or(&9999);
                if tentative_g_score < *neighbor_g_score {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + distance(neighbor, (0, 0, 0)));
                    if !open_set.contains(&neighbor) && !cubes.contains(&neighbor) {
                        open_set.insert(neighbor);
                    }
                }
            }
        }
    }
    return true;
}
fn distance(a: Cube, b: Cube) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}
fn cmp_option(a: Option<&i32>, b: Option<&i32>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (Some(x), Some(y)) => x.cmp(&y),
    }
}

// BFS was too slow :(
fn bfs(cubes: &HashSet<Cube>, start: Cube) -> bool {
    let target = (0, 0, 0);
    let mut queue = vec![start];
    let mut visited: HashSet<Cube> = HashSet::new();
    let adj = vec![
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, -1, 0),
        (-1, 0, 0),
    ];
    while !queue.is_empty() {
        let next = queue.remove(0);
        visited.insert(next);
        if next == target {
            return false;
        }
        for n in adj.iter() {
            let neigh = (next.0 + n.0, next.1 + n.1, next.2 + n.2);
            if !visited.contains(&neigh) && !cubes.contains(&neigh) {
                queue.push(neigh);
            }
        }
    }
    return true;
}

// For every space, do a search to find (0,0,0) (which doesn't have a cube in it)
// If it can reach that, it's not trapped, otherwise it is
fn trapped_spaces(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for cube in cubes.iter() {
        if cube.0 > max_x {
            max_x = cube.0
        }
        if cube.1 > max_y {
            max_y = cube.1
        }
        if cube.2 > max_z {
            max_z = cube.2
        }
    }
    let mut trapped: HashSet<Cube> = HashSet::new();
    for x in 0..max_x + 1 {
        for y in 0..max_y + 1 {
            for z in 0..max_z + 1 {
                let space = (x, y, z);
                if cubes.contains(&space) {
                    // This space is a cube, so can't be trapped
                    continue;
                }
                if a_star(&cubes, space) {
                    trapped.insert(space);
                }
            }
        }
    }
    trapped
}

fn count_free_sides(cubes: &HashSet<Cube>, trapped: &HashSet<Cube>) -> i32 {
    let adj = vec![
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, -1, 0),
        (-1, 0, 0),
    ];
    let mut free_sides = 0;
    for cube in cubes.iter() {
        for n in adj.iter() {
            let neighbor = (cube.0 + n.0, cube.1 + n.1, cube.2 + n.2);
            // Part 1 and 2 neatly contained here because trapped is just an empty set when part 1
            if !cubes.contains(&neighbor) && !trapped.contains(&neighbor) {
                free_sides += 1;
            }
        }
    }
    free_sides
}

fn main() {
    let cubes = read_input("input.txt");
    let free = count_free_sides(&cubes, &HashSet::new());
    println!("{:?}", free);

    let trapped = trapped_spaces(&cubes);
    let free = count_free_sides(&cubes, &trapped);
    println!("{:?}", free);
}
