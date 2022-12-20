use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input(file_name: &str) -> Vec<i32> {
    let mut numbs = Vec::new();
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let row = line.expect("Should be able to read line");
        numbs.push(row.parse::<i32>().expect("Should be parseable"));
    }
    numbs
}

// I assume all numbers are unique
fn mix(numbers: &Vec<i32>) -> VecDeque<i32> {
    let mut mixer: VecDeque<i32> = numbers.clone().into();
    for (_it, &n) in numbers.iter().enumerate() {
        //println!("n: {}, {:?}", n, mixer);
        if let Some(i) = mixer.iter().position(|&x| x == n) {
            mixer.rotate_left(i);
            let x = mixer.pop_front();
            let len = mixer.len() as i32;
            let mut j = n;
            while j < 0 {
                j += len;
            }
            while j >= len {
                j -= len;
            }
            //println!("{:?}", x);
            //println!("it: {}, i: {}, n: {}, j: {}, len: {}", _it, i, n, j, len);
            mixer.rotate_left(j as usize);
            mixer.push_back(n);
        }
    }
    //println!("{:?}", mixer);
    mixer
}

fn part1(numbers: &Vec<i32>) -> Option<i32> {
    let mixed = mix(&numbers);
    let mut cycle = mixed.iter().cycle();
    cycle.find(|&&x| x == 0);
    let mut ans = 0;
    for i in vec![1000, 2000, 3000] {
        let mut c = cycle.clone().skip(i - 1); // -1 because find starts _after_ what it found
        let next = c.next()?;
        println!("{}", next);
        ans += next;
    }
    Some(ans)
}

fn main() {
    let numbers = read_input("sample.txt");
    println!("{:?}", part1(&numbers));

    let numbers = read_input("input.txt");
    println!("{:?}", part1(&numbers));
}
