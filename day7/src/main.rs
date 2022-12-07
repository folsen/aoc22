use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

// One should be able to construct the total recursive size of a directory by
// counting "cd x" to "cd .." pairs. Every time you hit a "cd X" it's +1 and
// "cd .." is -1, when you're at 0, that's the recursive size of the folder you
// started at. This means you have to go through the list multiple times, but
// should still be relatively efficient. For root dir you need to read the
// whole list. Once you're done with a dir (hit 0 or end), you move forward to
// the next "cd X" and parse until you hit 0.

fn main() {
    let file = File::open("input.txt").expect("Can't open file");
    let reader = BufReader::new(file);
    // Read the whole file into memory for easier handling
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect();

    // Build a Vec of folders and their sizes
    let mut results: Vec<(String, i32)> = Vec::new();
    let re = Regex::new(r"(\d+) .*").unwrap();
    let mut tracker: i32 = 0;
    for (i, l1) in lines.iter().enumerate() {
        if &l1[0..4] == "$ cd" && &l1[0..] != "$ cd .." {
            let name = l1[5..].to_string();
            let mut size = 0;
            for l2 in &lines[i..] {
                if l2 == "$ cd .." {
                    tracker -= 1;
                } else if &l2[0..4] == "$ cd" {
                    tracker += 1;
                } else if re.is_match(l2) {
                    if let Some(caps) = re.captures(l2) {
                        size += caps.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0);
                    }
                }
                if tracker == 0 {
                    break;
                }
            }
            tracker = 0;
            results.push((name, size));
        }
    }

    // Part 1
    println!(
        "{:?}",
        results
            .iter()
            .fold(0, |acc, (_, size)| if *size <= 100_000 {
                acc + size
            } else {
                acc
            })
    );

    // Part 2
    let size_required = 30_000_000 - (70_000_000 - results[0].1);
    results.sort_by(|(_, a), (_, b)| a.cmp(b));
    println!(
        "{:?}",
        results.iter().find(|&&(_, size)| size > size_required)
    )
}
