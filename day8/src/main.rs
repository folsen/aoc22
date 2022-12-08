use std::fs::File;
use std::io::{prelude::*, BufReader};

// The forest is a matrix of dynamic size
// Indexing is done based on ease of parsing.
// Every inner vector is a row of the input file so the outer
// vector is the collection of rows. If you index forest[X][Y]
// then X is the row and Y is the column.
type Forest = Vec<Vec<i32>>;

fn read_input() -> std::io::Result<Forest> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut forest = Vec::new();
    for line in reader.lines() {
        let nums: Vec<i32> = line?
            .chars()
            .map(|c| c.to_string().parse::<i32>())
            .filter(|c| c.is_ok())
            .map(|c| c.expect("Already checked is_ok"))
            .collect();
        forest.push(nums);
    }
    Ok(forest)
}

fn check_visibility(x: usize, y: usize, forest: &Forest) -> bool {
    let my_height = forest[x][y];

    let visible_left = forest[x][0..y].iter().all(|v| *v < my_height);
    let visible_right = forest[x][y + 1..].iter().all(|v| *v < my_height);
    let visible_up = forest[0..x].iter().map(|r| r[y]).all(|v| v < my_height);
    let visible_down = forest[x + 1..].iter().map(|r| r[y]).all(|v| v < my_height);

    visible_left || visible_right || visible_up || visible_down
}

fn get_scenic_score(x: usize, y: usize, forest: &Forest) -> i32 {
    let my_height = forest[x][y];

    let mut visible_left = 0;
    for tree in forest[x][0..y].iter().rev() {
        visible_left += 1;
        if *tree >= my_height {
            break;
        }
    }
    let mut visible_right = 0;
    for tree in forest[x][y + 1..].iter() {
        visible_right += 1;
        if *tree >= my_height {
            break;
        }
    }
    let mut visible_up = 0;
    for tree in forest[0..x].iter().map(|r| r[y]).rev() {
        visible_up += 1;
        if tree >= my_height {
            break;
        }
    }
    let mut visible_down = 0;
    for tree in forest[x + 1..].iter().map(|r| r[y]) {
        visible_down += 1;
        if tree >= my_height {
            break;
        }
    }

    (visible_left * visible_right * visible_up * visible_down) as i32
}

fn main() {
    let forest = read_input().expect("Couldn't read file");
    let num_rows = forest.len();
    let num_cols = forest[0].len();
    let mut visible = 0; // Count of visible trees

    let mut highest_scenic_score = 0;
    for x in 0..num_rows {
        for y in 0..num_cols {
            if check_visibility(x, y, &forest) {
                visible += 1;
            }
            let score = get_scenic_score(x, y, &forest);
            if score > highest_scenic_score {
                highest_scenic_score = score;
            }
        }
    }

    // Part 1
    println!("{}", visible);

    // Part 2
    println!("{}", highest_scenic_score);
}
