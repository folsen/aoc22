use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::{Error, ErrorKind};

fn detect_signal(count: usize) -> io::Result<i32> {
    let file = File::open("input.txt")?;
    let bytes = BufReader::new(file).bytes();
    let mut signal: Vec<u8> = Vec::new();
    for (i, bit) in bytes.enumerate() {
        if i <= count - 1 {
            signal.push(bit?);
        } else {
            let check_set: HashSet<u8> = HashSet::from_iter(signal.iter().cloned());
            if check_set.len() == count {
                return Ok(i as i32);
            }
            signal.remove(0);
            signal.push(bit?);
        }
    }
    Err(Error::new(ErrorKind::Other, "Couldn't find a signal"))
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", detect_signal(4)?);
    println!("Part 2: {}", detect_signal(14)?);
    Ok(())
}
