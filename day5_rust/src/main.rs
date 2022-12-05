use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

type Stack = Vec<Vec<char>>;
type Instructions = Vec<(usize, usize, usize)>;

fn parse_input() -> io::Result<(Stack, Instructions)> {
    let mut stack = Vec::new();
    let mut instructions = Vec::new();
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let row = line?;
        if i < 8 {
            let cols: usize = row.len()/4 + 1;
            let row_chars = row.as_bytes();
            for i in 0..cols {
                let crate_name = row_chars[i*4+1] as char;
                match (stack.get_mut(i), crate_name) {
                    (None,' ') => stack.push(Vec::new()),
                    (Some(_),' ') => (),
                    (Some(col), _) => col.insert(0, crate_name),
                    (_,_) => stack.push(Vec::from([crate_name])),
                }
            }
        } else if i > 9 {
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
    Ok((stack, instructions))
}

fn execute_instructions_part1(stack: &mut Stack, instructions: &Instructions) -> Option<()> {
    for (count, from, to) in instructions {
        // Part 1
        for _ in 0..*count {
            let crate_name = stack.get_mut(*from)?.pop()?;
            stack.get_mut(*to)?.push(crate_name);
        }
    }
    Some(())
}

fn execute_instructions_part2(stack: &mut Stack, instructions: &Instructions) -> Option<()> {
    for (count, from, to) in instructions {
        let from_stack = stack.get_mut(*from)?;
        let mut moving_bits = from_stack.split_off(from_stack.len()-*count);
        stack.get_mut(*to)?.append(&mut moving_bits);
    }
    Some(())
}

fn print_result(stack: &Stack) {
    for col in stack {
        print!("{}", col.last().unwrap_or(&' '));
    }
    print!("\n")
}

fn main() -> io::Result<()> {
    let (mut stack1, instructions) = parse_input()?;
    let mut stack2 = stack1.clone();

    execute_instructions_part1(&mut stack1, &instructions);
    print_result(&stack1);
    execute_instructions_part2(&mut stack2, &instructions);
    print_result(&stack2);

    Ok(())
}