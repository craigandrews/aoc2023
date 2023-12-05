use std::{collections::HashMap, io};

fn main() {
    part2();
}

fn map_value(mapping: &Vec<(i64, i64, i64)>, v: i64) -> i64 {
    for m in mapping {
        if v < m.0 {
            return v;
        }

        if v >= m.0 && v <= m.1 {
            return v + m.2;
        }
    }
    return v;
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn part2() {
    let mut seeds: Vec<(i64, i64)> = Default::default();
    let mut mappings: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut current_map: String = Default::default();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.starts_with("seeds:") {
            let values: Vec<i64> = line
                .splitn(2, " ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            for pair in values.chunks(2) {
                let &[a, b] = pair else { todo!() };
                seeds.push((a, a + b - 1));
            }
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
            let mapping = mappings.get_mut(&current_map).unwrap();
            mapping.push((start, end, diff));
            mapping.sort();
        }
    }

    let seed_to_soil = mappings.get("seed-to-soil").unwrap();
    let soil_to_fertilizer = mappings.get("soil-to-fertilizer").unwrap();
    let fertilizer_to_water = mappings.get("fertilizer-to-water").unwrap();
    let water_to_light = mappings.get("water-to-light").unwrap();
    let light_to_temperature = mappings.get("light-to-temperature").unwrap();
    let temperature_to_humidity = mappings.get("temperature-to-humidity").unwrap();
    let humidity_to_location = mappings.get("humidity-to-location").unwrap();

    let mut lowest_location: Option<i64> = None;
    let mut counter: i64 = 0;
    for seed_pair in seeds {
        println!("{:?}", seed_pair);
        for seed in seed_pair.0..seed_pair.1 {
            counter += 1;
            if counter % 1000000 == 0 {
                println!("{}", counter)
            }

            let location = map_value(
                humidity_to_location,
                map_value(
                    temperature_to_humidity,
                    map_value(
                        light_to_temperature,
                        map_value(
                            water_to_light,
                            map_value(
                                fertilizer_to_water,
                                map_value(soil_to_fertilizer, map_value(seed_to_soil, seed)),
                            ),
                        ),
                    ),
                ),
            );

            if lowest_location.is_none() || location < lowest_location.unwrap() {
                lowest_location = Some(location);
            }
        }
    }

    println!("{}", lowest_location.unwrap());
}
