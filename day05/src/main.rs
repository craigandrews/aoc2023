use std::{collections::HashMap, io};

fn main() {
    part1();
}

fn map_value(mapping: &Vec<(i64, i64, i64)>, v: i64) -> i64 {
    for m in mapping {
        if v >= m.0 && v <= m.1 {
            return v + m.2;
        }
    }
    return v;
}

fn part1() {
    let mut seeds: Vec<i64> = Default::default();
    let mut mappings: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut current_map: String = Default::default();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.starts_with("seeds:") {
            seeds = line
                .splitn(2, " ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
        } else if line.ends_with("map:") {
            current_map = line.split_whitespace().nth(0).unwrap().to_owned();
            mappings.insert(current_map.clone(), Default::default());
        } else if !line.is_empty() {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            let start = values[1];
            let end = values[1] + values[2] - 1;
            let diff: i64 = values[0] - values[1];
            let t = (start, end, diff);
            let mapping = mappings.get_mut(&current_map).unwrap();
            mapping.push(t);
        }
    }

    let mut lowest_location: Option<i64> = None;
    for seed in seeds {
        let soil = map_value(mappings.get("seed-to-soil").unwrap(), seed);
        let fertilizer = map_value(mappings.get("soil-to-fertilizer").unwrap(), soil);
        let water = map_value(mappings.get("fertilizer-to-water").unwrap(), fertilizer);
        let light = map_value(mappings.get("water-to-light").unwrap(), water);
        let temperature = map_value(mappings.get("light-to-temperature").unwrap(), light);
        let humidity = map_value(
            mappings.get("temperature-to-humidity").unwrap(),
            temperature,
        );
        let location = map_value(mappings.get("humidity-to-location").unwrap(), humidity);

        if lowest_location.is_none() || location < lowest_location.unwrap() {
            lowest_location = Some(location);
        }
    }

    println!("{}", lowest_location.unwrap());
}
