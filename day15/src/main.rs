use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Device {
    Sensor(Pos, i32), // If it's a sensor, there's a distance to a beacon
    Beacon(Pos),
}
type Pos = (i32, i32);

fn distance(p1: Pos, p2: Pos) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn read_input() -> HashSet<Device> {
    let re = Regex::new(r"[\w\s]+ x=(-?\d+), y=(-?\d+):[\w\s]+ x=(-?\d+), y=(-?\d+)").unwrap();
    let mut devices = HashSet::new();
    let file = File::open("input.txt").expect("Couldn't open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        for cap in re.captures_iter(&line.expect("Couldn't read line")) {
            let is: Vec<i32> = vec![&cap[1], &cap[2], &cap[3], &cap[4]]
                .iter()
                .map(|i| i.parse::<i32>().expect("Should parse int"))
                .collect();
            let s = (is[0], is[1]);
            let b = (is[2], is[3]);
            let d = distance(s, b);
            devices.insert(Device::Sensor(s, d));
            devices.insert(Device::Beacon(b));
        }
    }
    devices
}

// Count the number of occupied spaces in the given row
// Stupid slow solution but it works, not for part 2 though
fn count_occupied(devices: &HashSet<Device>, y: i32) -> i32 {
    // Find the min and max x for the set, so we know what to iterate between
    let xs: Vec<&i32> = devices
        .iter()
        .map(|d| match d {
            Device::Sensor((x, _), _) => x,
            Device::Beacon((x, _)) => x,
        })
        .collect();
    let min_x = **xs.iter().min().expect("Should be a min");
    let max_x = **xs.iter().max().expect("Should be a max");
    let max_dist = devices
        .iter()
        .map(|d| match d {
            Device::Sensor(_, dist) => *dist,
            Device::Beacon((_, _)) => 0,
        })
        .max()
        .expect("Should exist a max distance");

    // For every point on the row, iterate through all the sensors and check
    // if the distance is (strictly) greater than the distance to its beacon
    // If this check is true for all sensors then this point is unchecked by any sensor
    // otherwise it is checked and ruled out for a beacon.
    let mut count = 0;
    for x in (min_x - max_dist)..(max_x + max_dist) {
        // Don't count if it's a beacon
        if devices.contains(&Device::Beacon((x, y))) {
            continue;
        }
        for d in devices.iter() {
            match d {
                Device::Sensor(sp, dist) => {
                    if distance((x, y), *sp) <= *dist {
                        count += 1;
                        // Found a device within reach so stop counting
                        break;
                    }
                }
                Device::Beacon(_) => (),
            }
        }
    }
    count
}

// Had to do the smart thing for part 2
fn check_range(devices: &HashSet<Device>, y: i32) -> Option<Pos> {
    let mut ranges: Vec<(i32, i32)> = Vec::new();
    for d in devices.iter() {
        match d {
            Device::Sensor((sx, sy), dist) => {
                if (sy - y).abs() <= *dist {
                    // If the distance between where we are and the sensor in y is greater than the distance
                    // To the beacon then this sensor isn't in range of where we are and shouldn't add the range at all
                    let x_left = sx - (dist - (sy - y).abs()).max(0);
                    let x_right = sx + (dist - (sy - y).abs()).max(0);
                    ranges.push((x_left, x_right));
                }
            }
            Device::Beacon(_) => (),
        }
    }
    ranges.sort();
    let first = ranges.remove(0);
    let min_x = first.0;
    let mut max_x = first.1;
    // If it's on the left we already found it if min_x is within range
    if min_x > 0 {
        return Some((min_x - 1, y));
    }
    for r in ranges {
        // They are sorted so r.0 will always be larger than min_x
        // The order of the check here matters
        if r.0 > max_x {
            // There is a gap here, so return the position after the max range assuming there is just one gap
            if max_x < 4_000_000 {
                return Some((max_x + 1, y));
            }
        } else if r.1 > max_x {
            // Expand range right
            max_x = r.1;
        }
    }
    None
}

fn main() {
    let devices = read_input();

    // Part 1
    let count = count_occupied(&devices, 2_000_000);
    println!("{:?}", count);

    // Part 2
    for y in 0..4_000_000 {
        if let Some(pos) = check_range(&devices, y) {
            println!("{:?}", (pos.0 as i128) * 4000000 + (pos.1 as i128));
            break;
        }
    }
}
