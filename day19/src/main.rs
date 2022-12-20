use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Blueprint {
    index: i32,           // the blueprint number, needed for output calc
    ore: i32,             // # of ore
    clay: i32,            // # of ore
    obsidian: (i32, i32), // # of ore and clay
    geode: (i32, i32),    // # of ore and obsidian
}

fn read_input(file_name: &str) -> Vec<Blueprint> {
    let file = File::open(file_name.to_string()).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut blueprints = Vec::new();
    let re = Regex::new(r"Blueprint (\d+).* (\d+) ore.* (\d+) ore.* (\d+) ore.* (\d+) clay.* (\d+) ore.* (\d+) obsidian").unwrap();
    for line in reader.lines() {
        for cap in re.captures_iter(&line.expect("Couldn't read line")) {
            blueprints.push(Blueprint {
                index: cap[1].parse::<i32>().unwrap(),
                ore: cap[2].parse::<i32>().unwrap(),
                clay: cap[3].parse::<i32>().unwrap(),
                obsidian: (
                    cap[4].parse::<i32>().unwrap(),
                    cap[5].parse::<i32>().unwrap(),
                ),
                geode: (
                    cap[6].parse::<i32>().unwrap(),
                    cap[7].parse::<i32>().unwrap(),
                ),
            });
        }
    }
    blueprints
}

// State is minutes, resources, bots
type State = (i32, (i32, i32, i32, i32), (i32, i32, i32, i32));

// Resources are (ore, clay, obsidian, geode)
// Robots are (ore, clay, obsidian, geode)
fn find_optimal_geodes(
    blueprint: &Blueprint,
    resources: (i32, i32, i32, i32),
    robots: (i32, i32, i32, i32),
    minute: i32,
    max_minutes: i32,
) -> Vec<State> {
    let mut queue: VecDeque<State> = VecDeque::from([(minute, resources, robots)]);
    let mut results = Vec::new();
    let mut seen: HashMap<((i32, i32, i32, i32), (i32, i32, i32, i32)), i32> = HashMap::new();
    while !queue.is_empty() {
        let next = queue.pop_front().expect("Queue not empty");
        //println!("{:?}", next);
        let (m, (ore, clay, obsidian, geode), (orebot, claybot, obsidianbot, geodebot)) = next;
        if seen.contains_key(&(next.1, next.2)) && seen[&(next.1, next.2)] <= m {
            continue;
        } else {
            seen.insert((next.1, next.2).clone(), m);
        }
        if m == max_minutes {
            // Record any positive result if hit time
            if geode > 0 {
                results.push(next);
            }
            continue;
        }
        let q_len = queue.len();
        let new_re = (
            ore + orebot,
            clay + claybot,
            obsidian + obsidianbot,
            geode + geodebot,
        );

        // I want a steady stream of resources to max the geode bot production.
        // Meaning at the end of the pipeline i need geode.0 and geode.1 ore and
        // obsidian produces to make geode bots I need obsidian.0 ore and
        // obsidian.1 clay to make clay bots continuously and then I need clay
        // ore for the obsidian bots

        // So trickling down, we need no more than geode.1 obsidian bots
        // no more than obsidian.1 clay bots
        // and no more than (geode.0 + obsidian.0 + clay) ore bots
        // Though realistically we will need quite a bit less than that because
        // we won't reach that steady-state and might want to over- or
        // under-produce bots to optimize.

        let max_geode_ore_consumption =
            blueprint.geode.0 * (max_minutes - m) + blueprint.obsidian.0 * (max_minutes - m);
        let max_geode_clay_consumption = blueprint.obsidian.1 * (max_minutes - m);

        // Build ore-bot
        let ore_consumption = (blueprint.geode.0 * (geodebot + 1)
            + blueprint.obsidian.0 * (obsidianbot + 1)
            + blueprint.clay * (claybot + 1))
            * (max_minutes - m);
        let ore_production = orebot * (max_minutes - m) + ore;
        if ore >= blueprint.ore
            && ore_production <= ore_consumption
            && ore_production <= max_geode_ore_consumption
        {
            let updated_ores = (new_re.0 - blueprint.ore, new_re.1, new_re.2, new_re.3);
            let updated_bots = (orebot + 1, claybot, obsidianbot, geodebot);
            queue.push_back((m + 1, updated_ores, updated_bots));
        }

        // Build clay bot
        let clay_consumption =
            blueprint.obsidian.1 * (obsidianbot + 1) * (max_minutes - m) + blueprint.obsidian.1;
        let clay_production = claybot * (max_minutes - m) + clay;
        if ore >= blueprint.clay
            && clay_production <= clay_consumption
            && clay_production <= max_geode_clay_consumption
        {
            let updated_ores = (new_re.0 - blueprint.clay, new_re.1, new_re.2, new_re.3);
            let updated_bots = (orebot, claybot + 1, obsidianbot, geodebot);
            queue.push_back((m + 1, updated_ores, updated_bots));
        }

        // Build obisidian bot
        let obsidian_consumption = blueprint.geode.1 * (geodebot + 1) * (max_minutes - m);
        let obsidian_production = obsidianbot * (max_minutes - m) + obsidian;
        if ore >= blueprint.obsidian.0
            && clay >= blueprint.obsidian.1
            && obsidian_production <= obsidian_consumption
        {
            let updated_ores = (
                new_re.0 - blueprint.obsidian.0,
                new_re.1 - blueprint.obsidian.1,
                new_re.2,
                new_re.3,
            );
            let updated_bots = (orebot, claybot, obsidianbot + 1, geodebot);
            queue.push_back((m + 1, updated_ores, updated_bots));
        }
        // Build geode bot
        if ore >= blueprint.geode.0 && obsidian >= blueprint.geode.1 {
            let updated_ores = (
                new_re.0 - blueprint.geode.0,
                new_re.1,
                new_re.2 - blueprint.geode.1,
                new_re.3,
            );
            let updated_bots = (orebot, claybot, obsidianbot, geodebot + 1);
            queue.push_back((m + 1, updated_ores, updated_bots));
        }
        // If we can't do anything we need to save
        // If we can spend on any of the above, then we have every option and shouldn't save
        if q_len == queue.len() || queue.len() - q_len != 4 {
            queue.push_back((m + 1, new_re, next.2));
        }
        //println!("{}", queue.len());
    }
    results
}

fn part1(blueprints: &Vec<Blueprint>) {
    let mut res = 0;
    for b in blueprints {
        println!("{:?}", b);
        let all = find_optimal_geodes(&b, (0, 0, 0, 0), (1, 0, 0, 0), 1, 25);
        let best = all.iter().max_by(|x, y| x.1 .3.cmp(&y.1 .3)).unwrap_or(&(
            0,
            (0, 0, 0, 0),
            (0, 0, 0, 0),
        ));
        println!("geodes: {}", (best.1).3);
        res += b.index * (best.1).3;
    }
    println!("{:?}", res);
}

fn part2(blueprints: &Vec<Blueprint>) {
    let mut res = 1;
    for b in blueprints.iter().take(3) {
        println!("{:?}", b);
        let all = find_optimal_geodes(&b, (0, 0, 0, 0), (1, 0, 0, 0), 1, 32);
        //println!("{:?}", all);
        let best = all.iter().max_by(|x, y| x.1 .3.cmp(&y.1 .3)).unwrap_or(&(
            0,
            (0, 0, 0, 0),
            (0, 0, 0, 0),
        ));
        println!("geodes: {}", (best.1).3);
        res *= (best.1).3;
    }
    println!("{:?}", res);
}

fn main() {
    let blueprints = read_input("sample.txt");
    //part1(&blueprints);
    part2(&blueprints);
}
