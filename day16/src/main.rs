use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{prelude::*, BufReader};

type Graph = HashMap<String, (i32, Vec<String>)>;
type DistanceMap = HashMap<String, HashMap<String, i32>>;

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

fn calc_distances(valves: &Graph) -> DistanceMap {
    let mut distances = HashMap::new();

    for from in valves.keys() {
        let mut tos = HashMap::new();
        for to in valves.keys() {
            let mut queue = VecDeque::from([(from, 0)]);
            let mut seen: HashSet<String> = HashSet::new();
            while !queue.is_empty() {
                let (curr, dist) = queue.pop_front().expect("Queue can't be empty here");
                seen.insert(curr.clone());
                if curr == to {
                    tos.insert(to.clone(), dist);
                    break;
                }
                for n in valves[curr].1.iter() {
                    if !seen.contains(n) {
                        queue.push_back((n, dist + 1));
                    }
                }
            }
        }
        distances.insert(from.clone(), tos);
    }
    distances
}

fn find_optimal_flow(valves: &Graph, dmap: &DistanceMap) -> i32 {
    let initial_valves: Vec<String> = valves
        .iter()
        .filter(|(_, v)| v.0 > 0)
        .map(|(k, _)| k.clone())
        .collect();

    let mut queue = VecDeque::from([("AA".to_string(), 30, 0, initial_valves)]);
    let mut best_flow = 0;
    while !queue.is_empty() {
        let (curr, min_left, flow, open_valves) = queue.pop_back().expect("not empty");
        // Open the valve here already subtracted -1 here when moving here
        let new_flow = flow + min_left * valves[&curr].0;
        best_flow = best_flow.max(new_flow);
        for (i, next) in open_valves.iter().enumerate() {
            // -1 to open current valve and -distance to next
            let new_min_left = min_left - 1 - dmap[&curr][next];
            if new_min_left >= 0 {
                let mut next_v = open_valves.clone();
                next_v.remove(i);
                queue.push_back((next.clone(), new_min_left, new_flow, next_v))
            }
        }
    }
    best_flow
}

fn find_optimal_double_flow(valves: &Graph, dmap: &DistanceMap) -> i32 {
    let initial_valves: Vec<String> = valves
        .iter()
        .filter(|(_, v)| v.0 > 0)
        .map(|(k, _)| k.clone())
        .collect();

    let mut queue = VecDeque::from([(
        "AA".to_string(),
        "AA".to_string(),
        26,
        26,
        0,
        initial_valves,
    )]);
    let mut seen = HashSet::new();
    let mut best_flow = 0;
    while !queue.is_empty() {
        let (man, ele, man_min_left, ele_min_left, flow, open_valves) =
            queue.pop_back().expect("not empty");

        let mut hasher = DefaultHasher::new();
        let st = (man.clone(), ele.clone(), flow, open_valves.clone());
        st.hash(&mut hasher);
        let k = hasher.finish();
        if seen.contains(&k) {
            continue;
        } else {
            seen.insert(k);
        }

        best_flow = best_flow.max(flow);

        for (i, next) in open_valves.iter().enumerate() {
            let new_man_min_left = man_min_left - 1 - dmap[&man][next];
            let new_flow = flow + new_man_min_left * valves[next].0;
            if new_man_min_left >= 0 {
                let mut next_v = open_valves.clone();
                next_v.remove(i);
                queue.push_back((
                    next.clone(),
                    ele.clone(),
                    new_man_min_left,
                    ele_min_left,
                    new_flow,
                    next_v.clone(),
                ))
            }
        }

        for (i, next) in open_valves.iter().enumerate() {
            let new_ele_min_left = ele_min_left - 1 - dmap[&ele][next];
            let new_flow = flow + new_ele_min_left * valves[next].0;
            if new_ele_min_left >= 0 {
                let mut next_v = open_valves.clone();
                next_v.remove(i);
                queue.push_back((
                    man.clone(),
                    next.clone(),
                    man_min_left,
                    new_ele_min_left,
                    new_flow,
                    next_v.clone(),
                ))
            }
        }
    }
    best_flow
}

fn main() {
    let valves = read_input("input.txt");
    let dmap = calc_distances(&valves);

    // Part 1
    let ans = find_optimal_flow(&valves, &dmap);
    println!("{:?}", ans);

    // Part 2
    let ans2 = find_optimal_double_flow(&valves, &dmap);
    println!("{:?}", ans2);
}
