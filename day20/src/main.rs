use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input(file_name: &str) -> Vec<(i64, i64)> {
    let mut numbs = Vec::new();
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let row = line.expect("Should be able to read line");
        numbs.push((i as i64, row.parse::<i64>().expect("Should be parseable")));
    }
    numbs
}

fn mix(numbers: &Vec<(i64, i64)>, iters: i64) -> VecDeque<(i64, i64)> {
    let mut mixer: VecDeque<(i64, i64)> = numbers.clone().into();
    for _ in 0..iters {
        for (idx, _) in numbers.iter() {
            let i = mixer.iter().position(|&(k, _)| k == *idx).unwrap();
            mixer.rotate_left(i);
            let x = mixer.pop_front().unwrap();
            let j = x.1.rem_euclid(mixer.len() as i64);
            mixer.rotate_left(j as usize);
            mixer.push_front(x);
        }
    }
    mixer
}

fn get_answer(numbers: &Vec<(i64, i64)>, iters: i64) -> Option<i64> {
    let mut mixed = mix(&numbers, iters);
    let z_pos = mixed.iter().position(|&(_, x)| x == 0).unwrap();
    mixed.rotate_left(z_pos);
    let mut ans = 0;
    for i in vec![1000, 2000, 3000] {
        let (_, next) = mixed[i % mixed.len()];
        println!("{}", next);
        ans += next;
    }
    Some(ans)
}

fn main() {
    // Part 1
    let numbers = read_input("input.txt");
    println!("{:?}", get_answer(&numbers, 1));

    // Part 2
    let part2_numbers = numbers
        .clone()
        .iter()
        .map(|(i, v)| (*i, v * 811589153))
        .collect();
    println!("{:?}", get_answer(&part2_numbers, 10))
}
