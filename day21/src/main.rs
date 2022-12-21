use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Constants = HashMap<String, i128>;
type Operators = HashMap<String, (String, String, String)>;

fn read_input(file_name: &str) -> (Constants, Operators) {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut constants = HashMap::new();
    let mut operators = HashMap::new();
    for line in reader.lines() {
        let row = line.expect("Should read line");
        let stripped = row.replace(":", "");
        let words: Vec<&str> = stripped.split(" ").collect();
        if let Ok(i) = words[1].parse::<i128>() {
            constants.insert(words[0].into(), i);
        } else {
            operators.insert(
                words[0].into(),
                (words[1].into(), words[2].into(), words[3].into()),
            );
        }
    }
    (constants, operators)
}

// Returns what "root" calculates
fn solve1(input: &mut (Constants, Operators)) -> i128 {
    let (constants, operators) = input;

    while !operators.is_empty() {
        for (monkey, (a, o, b)) in operators.iter_mut() {
            if constants.contains_key(a) && constants.contains_key(b) {
                let val = match &o[0..] {
                    "+" => constants[a] + constants[b],
                    "-" => constants[a] - constants[b],
                    "*" => constants[a] * constants[b],
                    "/" => constants[a] / constants[b],
                    _ => 0,
                };
                constants.insert(monkey.clone(), val);
            }
        }
        operators.retain(|k, _| !constants.contains_key(k));
    }

    constants["root".into()]
}

fn solve2(input: &mut (Constants, Operators)) -> i128 {
    let (constants, operators) = input;

    while !operators.is_empty() {
        let in_len = operators.len();
        for (monkey, (a, o, b)) in operators.iter_mut() {
            if *monkey == "humn".to_string()
                || *monkey == "root".to_string()
                || *a == "humn".to_string()
                || *b == "humn".to_string()
            {
                continue;
            }
            if constants.contains_key(a) && constants.contains_key(b) {
                let val = match &o[0..] {
                    "+" => constants[a] + constants[b],
                    "-" => constants[a] - constants[b],
                    "*" => constants[a] * constants[b],
                    "/" => constants[a] / constants[b],
                    _ => 0,
                };
                constants.insert(monkey.clone(), val);
            }
        }
        operators.retain(|k, _| !constants.contains_key(k));
        if in_len == operators.len() {
            // We're not progressing anymore here
            break;
        }
    }
    let (left, _, right) = &operators[&"root".to_string()];
    if let Some(num) = constants.get(left) {
        constants.insert(right.clone(), *num);
    }
    if let Some(num) = constants.get(right) {
        constants.insert(left.clone(), *num);
    }
    operators.remove(&"root".to_string());
    constants.remove(&"humn".to_string());

    // Rewrite equations starting from human
    let mut to_rewrite = vec!["humn".to_string()];
    let mut rewritten: HashSet<String> = HashSet::new();
    while !to_rewrite.is_empty() {
        let mut to_insert: Option<(String, (String, String, String))> = None;
        let mut to_remove: Option<String> = None;
        let curr = to_rewrite.pop().unwrap();
        if rewritten.contains(&curr) {
            continue;
        }
        rewritten.insert(curr.clone());

        for (monkey, (a, o, b)) in operators.iter() {
            if rewritten.contains(monkey) {
                continue;
            }
            let lookups = (constants.get(monkey), constants.get(a), constants.get(b));
            if *a == curr {
                // monkey = humn `op` b
                if let (_, _, Some(_)) = lookups {
                    let new_op = match &o[0..] {
                        "+" => "-".to_string(),
                        "-" => "+".to_string(),
                        "*" => "/".to_string(),
                        "/" => "*".to_string(),
                        _ => "".to_string(),
                    };
                    to_rewrite.push(monkey.clone());
                    to_remove = Some(monkey.clone());
                    to_insert = Some((curr.clone(), (monkey.clone(), new_op, b.clone())));
                }
                break;
            }
            if *b == curr {
                // monkey = a `op` humn
                if let (_, Some(_), _) = lookups {
                    let entry = match &o[0..] {
                        "+" => (monkey.clone(), "-".into(), a.clone()),
                        "-" => (a.clone(), "-".into(), monkey.clone()),
                        "*" => (monkey.clone(), "/".into(), a.clone()),
                        "/" => (a.clone(), "/".into(), monkey.clone()),
                        _ => ("".into(), "".into(), "".into()),
                    };
                    to_rewrite.push(monkey.clone());
                    to_remove = Some(monkey.clone());
                    to_insert = Some((curr.clone(), entry));
                }
                break;
            }
        }
        if let Some(x) = to_insert {
            operators.insert(x.0, x.1);
        }
        if let Some(x) = to_remove {
            operators.remove(&x);
        }
    }

    loop {
        let in_len = operators.len();

        for (monkey, (a, o, b)) in operators.iter() {
            let lookups = (constants.get(monkey), constants.get(a), constants.get(b));
            match lookups {
                (Some(lhs), Some(lrhs), None) => {
                    let val = match &o[0..] {
                        "+" => lhs - lrhs,
                        "-" => lrhs - lhs,
                        "*" => lhs / lrhs,
                        "/" => lrhs / lhs,
                        _ => 0,
                    };
                    constants.insert(a.clone(), val);
                }
                (Some(lhs), None, Some(rrhs)) => {
                    let val = match &o[0..] {
                        "+" => lhs - rrhs,
                        "-" => lhs + rrhs,
                        "*" => lhs / rrhs,
                        "/" => lhs * rrhs,
                        _ => 0,
                    };
                    constants.insert(b.clone(), val);
                }
                (None, Some(lrhs), Some(rrhs)) => {
                    let val = match &o[0..] {
                        "+" => lrhs + rrhs,
                        "-" => lrhs - rrhs,
                        "*" => lrhs * rrhs,
                        "/" => lrhs / rrhs,
                        _ => 0,
                    };
                    constants.insert(monkey.clone(), val);
                }
                _ => (),
            }
        }
        operators.retain(|k, _| !constants.contains_key(k));
        if in_len == operators.len() {
            // We're not progressing anymore here
            break;
        }
    }
    constants["humn".into()]
}

fn main() {
    let mut input = read_input("input.txt");
    let mut input2 = input.clone();

    // Part 1
    let root = solve1(&mut input);
    println!("{:?}", root);

    // Part 2
    let humn = solve2(&mut input2);
    println!("{:?}", humn);
}
