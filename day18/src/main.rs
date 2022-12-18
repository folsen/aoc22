use std::collections::HashSet;
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

// BFS was too slow but DFS works super fast
fn non_trapped(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let maxes = bounds(&cubes);
    let start = (0, 0, 0);
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
        let next = queue.pop().expect("not empty");
        visited.insert(next);
        for n in adj.iter() {
            let neigh = (next.0 + n.0, next.1 + n.1, next.2 + n.2);
            if !visited.contains(&neigh)
                && !cubes.contains(&neigh)
                && neigh.0 < maxes.0 + 1
                && neigh.1 < maxes.1 + 1
                && neigh.2 < maxes.2 + 1
                && neigh.0 >= -1
                && neigh.1 >= -1
                && neigh.2 >= -1
            {
                queue.push(neigh);
            }
        }
    }
    visited
}

fn bounds(cubes: &HashSet<Cube>) -> (i32, i32, i32) {
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
    (max_x + 1, max_y + 1, max_z + 1)
}

fn count_free_sides(part: i32, cubes: &HashSet<Cube>, free_cubes: &HashSet<Cube>) -> i32 {
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
            if part == 1 && !cubes.contains(&neighbor) {
                free_sides += 1;
            }
            if part == 2 && !cubes.contains(&neighbor) && free_cubes.contains(&neighbor) {
                free_sides += 1;
            }
        }
    }
    free_sides
}

fn main() {
    let cubes = read_input("input.txt");
    let free = count_free_sides(1, &cubes, &HashSet::new());
    println!("{:?}", free);

    let free_cubes = non_trapped(&cubes);
    let free_sides = count_free_sides(2, &cubes, &free_cubes);
    println!("{:?}", free_sides);
}
