use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Graph = HashMap<String, (i32, Vec<String>)>;

fn read_input(file_name: String) -> Graph {
    let mut valves = HashMap::new();
    let file = File::open(file_name).expect("Couldn't open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let row = line.expect("Should be able to read line");
        let words = row.split(" ").collect::<Vec<&str>>();
        let name = words[1];
        let flow = words[4].replace(";", "").split("=").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .expect("Should convert to int");
        let edges = row
            .split(" ")
            .skip(9)
            .map(|s| s.to_string().replace(",", ""))
            .collect::<Vec<String>>();
        valves.insert(name.to_string(), (flow, edges));
    }
    valves
}

// The condition to go back or not is that the total_flow now must be higher than what it was
// When I visited the node last time, or we had no benefit from going back to this point
fn dfs(
    valves: &Graph,
    visited: &mut HashMap<String, i32>,
    opened: &mut HashSet<String>,
    results: &mut Vec<i32>,
    minutes_left: i32,
    total_flow: i32,
    curr: String,
) {
    visited.insert(curr.clone(), total_flow);
    if valves[&curr].0 == 0 {
        // Consider this valve opened without wasting time on opening it
        opened.insert(curr.clone());
    }
    if minutes_left <= 0 {
        return results.push(total_flow);
    } else if opened.len() == valves.len() {
        return results.push(total_flow);
    }

    // Our "moves" from here are to open if it's not opened and restart the iteration on the same place
    // Or two not open and then move to one of the neighbors

    if !opened.contains(&curr) {
        let mut new_o = opened.clone();
        new_o.insert(curr.clone());
        let new_flow = total_flow + (minutes_left - 1) * valves[&curr].0;
        // minutes_left - 1 since it takes a minute to open the valve
        dfs(
            valves,
            &mut visited.clone(),
            &mut new_o.clone(),
            results,
            minutes_left - 1, // One minute to open
            new_flow,
            curr.clone(),
        );
    }
    for n in valves[&curr].1.iter() {
        // If we've already been there, but we're now at a higher flow, we can go back
        // If we haven't been there, we can go there
        if visited.contains_key(n) && visited[n] < total_flow || !visited.contains_key(n) {
            dfs(
                valves,
                &mut visited.clone(),
                &mut opened.clone(),
                results,
                minutes_left - 1,
                total_flow,
                n.to_string(),
            );
        }
    }
    results.push(total_flow);
}

//enum Move {
//    Open(String),
//    StepTo(String),
//}
//
//fn better_dfs(valves: &Graph, start: String) {
//    let mut queue = vec![Move::StepTo(start.clone())];
//    // Value is (opened, minutes_left, total_flow)
//    let mut visited: HashMap<String, (bool, i32, i32)> = HashMap::new();
//    visited.insert(start, (false, 30, 0));
//
//    while !queue.is_empty() {
//        let curr = queue.pop().expect("Queue can't be empty here");
//        match curr {
//            Move::Open(n) => (),
//            Move::StepTo(n) => (),
//        }
//        // In addition to the moves to other nodes, we have the option of opening a valve
//        // if it's visited and not opened or not visited, then open it
//
//        if !visited[&curr].0 {
//            queue.push(Move::Open(curr));
//            let new_flow = curr_node.2 + valves[&curr].0 * (curr_node.1 - 1);
//            visited.insert(curr.clone(), (true, curr_node.1 - 1, new_flow));
//        }
//        // Add all relevant neighbors to exploration
//        for n in valves[&curr].1.iter() {
//            if visited.contains_key(n) && visited[n].2 < visited[&curr].2
//                || !visited.contains_key(n)
//            {
//                queue.push(n.clone());
//            }
//        }
//    }
//}

fn main() {
    let mut visited: HashMap<String, i32> = HashMap::new();
    let mut opened: HashSet<String> = HashSet::new();
    let mut results: Vec<i32> = Vec::new();
    let valves = read_input("input.txt".to_string());
    dfs(
        &valves,
        &mut visited,
        &mut opened,
        &mut results,
        30,
        0,
        "AA".to_string(),
    );
    println!("{:?}", results.iter().max());
}
