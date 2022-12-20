use regex::Regex;
use std::collections::{HashSet, VecDeque};
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
fn find_optimal_geodes(blueprint: &Blueprint, max_minutes: i32) -> i32 {
    let mut queue: VecDeque<State> = VecDeque::from([(1, (0, 0, 0, 0), (1, 0, 0, 0))]);
    let mut best = 0;
    let mut seen: HashSet<State> = HashSet::new();
    while !queue.is_empty() {
        let next = queue.pop_front().expect("Queue not empty");
        //println!("{:?}", next);
        let (m, (ore, clay, obsidian, geode), (orebot, claybot, obsidianbot, geodebot)) = next;
        best = best.max(geode);
        if seen.contains(&next) {
            continue;
        } else {
            seen.insert(next);
        }
        if m == max_minutes {
            continue;
        }
        let new_re = (
            ore + orebot,
            clay + claybot,
            obsidian + obsidianbot,
            geode + geodebot,
        );

        let max_ore_consumption = [
            blueprint.ore,
            blueprint.clay,
            blueprint.obsidian.0,
            blueprint.geode.0,
        ]
        .iter()
        .max()
        .unwrap()
        .clone();
        let time_left = max_minutes - m - 1;

        // Save
        queue.push_back((m + 1, new_re, next.2));

        // Build ore-bot
        let ore_consumption = max_ore_consumption * time_left;
        let ore_production = orebot * time_left + ore;
        if ore >= blueprint.ore
            && orebot <= max_ore_consumption
            && ore_production <= ore_consumption
        {
            let updated_ores = (new_re.0 - blueprint.ore, new_re.1, new_re.2, new_re.3);
            let updated_bots = (orebot + 1, claybot, obsidianbot, geodebot);
            queue.push_back((m + 1, updated_ores, updated_bots));
        }

        // Build clay bot
        let clay_consumption = blueprint.obsidian.1 * time_left;
        let clay_production = claybot * time_left + clay;
        if ore >= blueprint.clay
            && claybot <= blueprint.obsidian.1
            && clay_production <= clay_consumption
        {
            let updated_ores = (new_re.0 - blueprint.clay, new_re.1, new_re.2, new_re.3);
            let updated_bots = (orebot, claybot + 1, obsidianbot, geodebot);
            queue.push_back((m + 1, updated_ores, updated_bots));
        }

        // Build obisidian bot
        let obsidian_consumption = blueprint.geode.1 * time_left;
        let obsidian_production = obsidianbot * time_left + obsidian;
        if ore >= blueprint.obsidian.0
            && clay >= blueprint.obsidian.1
            && obsidianbot <= blueprint.geode.1
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
    }
    best
}

fn part1(blueprints: &Vec<Blueprint>) {
    let mut res = 0;
    for b in blueprints {
        res += b.index * find_optimal_geodes(&b, 25);
    }
    println!("{:?}", res);
}

fn part2(blueprints: &Vec<Blueprint>) {
    let mut res = 1;
    for b in blueprints.iter().take(3) {
        res *= find_optimal_geodes(&b, 33);
    }
    println!("{:?}", res);
}

fn main() {
    let blueprints = read_input("input.txt");
    part1(&blueprints);
    part2(&blueprints);
}
