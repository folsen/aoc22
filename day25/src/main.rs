use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap_or("".to_string()))
        .collect::<Vec<String>>()
}

fn from_fivary(s: &String) -> i64 {
    let l = s.len();
    let mut res = 0;
    for (i, c) in s.chars().enumerate() {
        let power = l - i - 1;
        let m = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unexpected char"),
        };
        res += m * 5_i64.pow(power as u32);
    }
    res
}

fn to_fivary(target: i64) -> String {
    let mut max_power = 0;
    while 2 * 5_i64.pow(max_power) < target {
        max_power += 1;
    }
    let mut num = 0;
    let mut s = vec![];
    for p in 0..max_power + 1 {
        let p = 5_i64.pow(max_power - p);
        let nums = [
            (-2 * p, '='),
            (-1 * p, '-'),
            (0, '0'),
            (1 * p, '1'),
            (2 * p, '2'),
        ];

        let min_diff = nums
            .iter()
            .min_by(|a, b| {
                (target - (num + a.0))
                    .abs()
                    .cmp(&(target - (num + b.0)).abs())
            })
            .expect("some min");
        num += min_diff.0;
        s.push(min_diff.1);
    }
    s.iter().collect::<String>()
}

fn main() {
    let nums = read_input("input.txt");

    // Part 1
    let ans = nums.iter().map(|s| from_fivary(s)).sum();
    println!("{:?}", to_fivary(ans));
}

#[test]
fn converting_from() {
    let s = "1=-1=".to_string();
    assert_eq!(from_fivary(&s), 353);
}
#[test]
fn converting_to() {
    let s = "1=-1=".to_string();
    assert_eq!(to_fivary(353), s);
}
