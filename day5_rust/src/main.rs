use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut stack1: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i < 8 {
            let row = line?;
            let cols: usize = row.len()/4 + 1;
            let row_chars = row.as_bytes();
            for i in 0..cols {
                let crate_name = row_chars[i*4+1] as char;
                match (stack1.get_mut(i), crate_name) {
                    (None,' ') => stack1.push(Vec::new()),
                    (Some(_),' ') => (),
                    (Some(col), _) => col.insert(0, crate_name),
                    (_,_) => stack1.push(Vec::from([crate_name])),
                }
            }
        } else if i > 9 {
            let row = line?;
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            for digits in re.captures_iter(&row) {
                instructions.push(
                    (digits[1].parse::<usize>().unwrap_or(0),
                     digits[2].parse::<usize>().unwrap_or(0) - 1,
                     digits[3].parse::<usize>().unwrap_or(0) - 1
                    )
                )
            }
        }
    }
    let mut stack2 = stack1.clone();

    for (count, from, to) in instructions {
        // Part 1
        for _ in 0..count {
            let crate_name = if let Some(from_stack) = stack1.get_mut(from) {
                from_stack.pop()
            } else {
                None
            };
            match (crate_name, stack1.get_mut(to)) {
                (Some(name), Some(to_stack)) => to_stack.push(name),
                (_,_) => ()
            }
        }

        // Part 2
        let mut moving_bits = if let Some(from_stack) = stack2.get_mut(from) {
            from_stack.split_off(from_stack.len()-count)
        } else {
            Vec::new()
        };
        if let Some(to_stack) = stack2.get_mut(to) {
            to_stack.append(&mut moving_bits);
        }
    }
    for col in stack1 {
        print!("{}", col.last().expect("There should be something left to look at"));
    }
    println!("");
    for col in stack2 {
        print!("{}", col.last().expect("There should be something left to look at"));
    }
    println!("");
    Ok(())
}