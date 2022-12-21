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
    let mut seen: HashSet<(String, i32, ValveState)> = HashSet::new();
    let mut best = 0;

    let mut hit_time = 0;
    let mut seen_cnt = 0;
    let mut all_op = 0;

    while !queue.is_empty() {
        let (curr_node, minute, flow, valve_state) =
            queue.pop_back().expect("Queue can't be empty here");

        println!(
            "q: {:?}, s: {}, time: {}, seen: {}, opened: {}, all_op: {}, best: {}",
            queue.len(),
            seen.len(),
            hit_time,
            seen_cnt,
            all_op,
            flow,
            best
        );
        if minute == time {
            hit_time += 1;
            continue;
        }
        if seen.contains(&(curr_node.clone(), minute, valve_state.clone())) {
            seen_cnt += 1;
            continue;
        }
        seen.insert((curr_node.clone(), minute, valve_state.clone()));
        best = best.max(flow);

        // Nothing more to do if all valves are opened
        if valve_state.len() == valves.len() {
            all_op += 1;
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

#[derive(PartialEq, Eq, Hash, Clone)]
struct SeenState {
    curr: String,
    ele: String,
    min: i32,
    vs: ValveState,
}

// Alt strategy

// Build a matrix of distances from every node to every other node
// Go to the valve with the highest immediate benefit taking the distance to it
// into account and open that one, then from that point repeat until all opened
// That won't really work.. it's more beneficial to open some valve on the way
// to some higher value one rather than go directly to high value and then back
// to the low-value
fn both(valves: &Graph, time: i32) -> i32 {
    // queue item is Node, Time, Flow, Opened Valves
    let mut queue = VecDeque::from([(
        "AA".to_string(),
        "AA".to_string(),
        0,
        0,
        initial_valve_state(valves),
    )]);
    let mut seen: HashSet<SeenState> = HashSet::new();

    let mut best = 0;
    let mut skips = 0;

    while !queue.is_empty() {
        let q = queue.pop_back().expect("Queue can't be empty here");
        let mut seen_state = SeenState {
            curr: q.0,
            ele: q.1,
            min: q.2,
            vs: q.4,
        };
        let minute = q.2;
        let flow = q.3;
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
        if seen.contains(&seen_state) {
            skips += 1;
            continue;
        }
        seen.insert(seen_state.clone());
        best = best.max(flow);

        // Nothing more to do if all valves are opened
        if seen_state.vs.len() == valves.len() {
            continue;
        }

        // If we make it here, we haven't been here before in this state.
        // Check if the valve is opened and open it if not
        // Otherwise continue to move to each of the possible locations

        let mut opened_state = seen_state.vs.clone();
        let mut new_flow = flow;
        let mut man_opened = false;
        let mut ele_opened = false;

        if !opened_state
            .iter()
            .find(|(k, _)| *k == seen_state.curr)
            .is_some()
        {
            man_opened = true;
            opened_state.push((seen_state.curr.clone(), minute + 1));
            new_flow += (time - minute - 1) * valves[&seen_state.curr].0;
        }
        if !opened_state
            .iter()
            .find(|(k, _)| *k == seen_state.ele)
            .is_some()
        {
            ele_opened = true;
            opened_state.push((seen_state.ele.clone(), minute + 1));
            new_flow += (time - minute - 1) * valves[&seen_state.ele].0;
        }

        if ele_opened && !man_opened {
            // Ele stands still but man can move
            for next in valves[&seen_state.curr].1.iter() {
                let st = SeenState {
                    curr: next.to_string(),
                    ele: seen_state.ele.clone(),
                    min: seen_state.min.clone(),
                    vs: opened_state.clone(),
                };
                if !seen.contains(&st) {
                    queue.push_back((st.curr, st.ele, minute + 1, new_flow, st.vs));
                }
            }
        }
        if man_opened && !ele_opened {
            // Man stands still but ele can move
            for next in valves[&seen_state.curr].1.iter() {
                let st = SeenState {
                    curr: seen_state.curr.clone(),
                    ele: next.to_string(),
                    min: seen_state.min.clone(),
                    vs: opened_state.clone(),
                };
                if !seen.contains(&st) {
                    queue.push_back((st.curr, st.ele, minute + 1, new_flow, st.vs));
                }
            }
        }
        if man_opened && ele_opened {
            let st = SeenState {
                curr: seen_state.curr.clone(),
                ele: seen_state.ele.clone(),
                min: seen_state.min.clone(),
                vs: opened_state.clone(),
            };
            if !seen.contains(&st) {
                queue.push_back((st.curr, st.ele, minute + 1, new_flow, st.vs));
            }
        }

        for next_man in valves[&seen_state.curr].1.iter() {
            for next_ele in valves[&seen_state.ele].1.iter() {
                seen_state.curr = next_man.to_string();
                seen_state.ele = next_ele.to_string();
                if !seen.contains(&seen_state) {
                    queue.push_back((
                        next_man.clone(),
                        next_ele.clone(),
                        minute + 1,
                        flow,
                        seen_state.vs.clone(),
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
