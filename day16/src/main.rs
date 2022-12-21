use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Graph = HashMap<String, (i32, Vec<String>)>;
type ValveState = Vec<(String, i32)>;

fn read_input(file_name: &str) -> Graph {
    let mut valves = HashMap::new();
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
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

// By default open all the 0 valves because we don't want to waste time trying to open them
fn initial_valve_state(valves: &Graph) -> ValveState {
    valves
        .iter()
        .filter(|(_, v)| v.0 == 0)
        .map(|(k, _)| (k.clone(), 0))
        .collect()
}

fn better_dfs(valves: &Graph, time: i32) -> i32 {
    // queue item is Node, Time, Flow, Opened Valves
    let mut queue = VecDeque::from([("AA".to_string(), 0, 0, initial_valve_state(valves))]);
    let mut seen: HashSet<(String, ValveState)> = HashSet::new();
    let mut best = 0;

    while !queue.is_empty() {
        let (curr_node, minute, flow, valve_state) =
            queue.pop_front().expect("Queue can't be empty here");

        if minute == time {
            continue;
        }
        if seen.contains(&(curr_node.clone(), valve_state.clone())) {
            continue;
        }
        seen.insert((curr_node.clone(), valve_state.clone()));
        best = best.max(flow);

        // Nothing more to do if all valves are opened
        if valve_state.len() == valves.len() {
            continue;
        }

        // If we make it here, we haven't been here before in this state.
        // Check if the valve is opened and open it if not
        // Otherwise continue to move to each of the possible locations

        if !valve_state.iter().find(|(k, _)| *k == curr_node).is_some() {
            let mut opened_state = valve_state.clone();
            opened_state.push((curr_node.clone(), minute + 1));
            let new_flow = flow + (time - minute - 1) * valves[&curr_node].0;
            queue.push_back((curr_node.clone(), minute + 1, new_flow, opened_state));
        }

        for next in valves[&curr_node].1.iter() {
            queue.push_back((next.clone(), minute + 1, flow, valve_state.clone()));
        }
    }
    return best;
}

fn both(valves: &Graph, time: i32) -> i32 {
    // queue item is Node, Time, Flow, Opened Valves
    let mut queue = VecDeque::from([(
        "AA".to_string(),
        "AA".to_string(),
        0,
        0,
        initial_valve_state(valves),
    )]);
    let mut seen: HashSet<(String, String, ValveState)> = HashSet::new();

    let mut best = 0;
    let mut skips = 0;

    while !queue.is_empty() {
        let (curr_node, ele_node, minute, flow, valve_state) =
            queue.pop_front().expect("Queue can't be empty here");
        if seen.len() % 100000 == 0 {
            println!(
                "q: {:?}, s: {}, skips: {}, flow: {}, best: {}",
                queue.len(),
                seen.len(),
                skips,
                flow,
                best
            );
        }
        if minute == time {
            continue;
        }
        if seen.contains(&(curr_node.clone(), ele_node.clone(), valve_state.clone())) {
            skips += 1;
            continue;
        }
        seen.insert((curr_node.clone(), ele_node.clone(), valve_state.clone()));
        best = best.max(flow);

        // Nothing more to do if all valves are opened
        if valve_state.len() == valves.len() {
            continue;
        }

        // If we make it here, we haven't been here before in this state.
        // Check if the valve is opened and open it if not
        // Otherwise continue to move to each of the possible locations

        let mut opened_state = valve_state.clone();
        let mut new_flow = flow;
        let mut man_opened = false;
        let mut ele_opened = false;

        if !opened_state.iter().find(|(k, _)| *k == curr_node).is_some() {
            man_opened = true;
            opened_state.push((curr_node.clone(), minute + 1));
            new_flow += (time - minute - 1) * valves[&curr_node].0;
        }
        if !opened_state.iter().find(|(k, _)| *k == ele_node).is_some() {
            ele_opened = true;
            opened_state.push((ele_node.clone(), minute + 1));
            new_flow += (time - minute - 1) * valves[&ele_node].0;
        }

        if ele_opened && !man_opened {
            // Ele stands still but man can move
            for next in valves[&curr_node].1.iter() {
                if !seen.contains(&(next.clone(), ele_node.clone(), opened_state.clone())) {
                    queue.push_back((
                        next.clone(),
                        ele_node.clone(),
                        minute + 1,
                        new_flow,
                        opened_state.clone(),
                    ));
                }
            }
        }
        if man_opened && !ele_opened {
            // Man stands still but ele can move
            for next in valves[&ele_node].1.iter() {
                if !seen.contains(&(curr_node.clone(), next.clone(), opened_state.clone())) {
                    queue.push_back((
                        curr_node.clone(),
                        next.clone(),
                        minute + 1,
                        new_flow,
                        opened_state.clone(),
                    ));
                }
            }
        }
        if man_opened && ele_opened {
            if !seen.contains(&(curr_node.clone(), ele_node.clone(), opened_state.clone())) {
                queue.push_back((
                    curr_node.clone(),
                    ele_node.clone(),
                    minute + 1,
                    new_flow,
                    opened_state.clone(),
                ));
            }
        }

        for next_man in valves[&curr_node].1.iter() {
            for next_ele in valves[&ele_node].1.iter() {
                if !seen.contains(&(next_man.clone(), next_ele.clone(), valve_state.clone())) {
                    queue.push_back((
                        next_man.clone(),
                        next_ele.clone(),
                        minute + 1,
                        flow,
                        valve_state.clone(),
                    ));
                }
            }
        }
    }
    println!("best: {}", best);
    return best;
}

fn main() {
    let valves = read_input("input.txt");

    // Part 1
    //let ans = better_dfs(&valves, 30);
    //println!("{:?}", ans);

    // Part 2
    let ans2 = both(&valves, 26);
    println!("{:?}", ans2);
}
