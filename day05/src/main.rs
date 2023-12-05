use std::io;

fn main() {
    part2();
}

#[allow(dead_code)]
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
    let mut mappings: Vec<Vec<(i64, i64, i64)>> = Default::default();
    let mut tmp: Vec<(i64, i64, i64)> = Default::default();
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
            if !tmp.is_empty() {
                tmp.sort();
                mappings.push(tmp);
                tmp = Default::default();
            }
        } else if !line.is_empty() {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            let start = values[1];
            let end = values[1] + values[2] - 1;
            let diff: i64 = values[0] - values[1];
            tmp.push((start, end, diff));
        }
    }

    if !tmp.is_empty() {
        tmp.sort();
        mappings.push(tmp);
    }

    let mut lowest_location: Option<i64> = None;
    for seed in seeds {
        let mut value: i64 = seed;
        for mapping in mappings.iter() {
            value = map_value(&mapping, value);
        }

        if lowest_location.is_none() || value < lowest_location.unwrap() {
            lowest_location = Some(value);
        }
    }

    println!("{}", lowest_location.unwrap());
}

#[allow(dead_code)]
fn unmap_value(mapping: &Vec<(i64, i64, i64)>, v: i64) -> i64 {
    for m in mapping {
        let x = v - m.2;

        if x >= m.0 && x <= m.1 {
            return x;
        }
    }
    return v;
}

#[allow(dead_code)]
fn part2() {
    let mut seeds: Vec<(i64, i64)> = Default::default();

    let mut mappings: Vec<Vec<(i64, i64, i64)>> = Default::default();
    let mut tmp: Vec<(i64, i64, i64)> = Default::default();
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
            seeds.sort();
        } else if line.ends_with("map:") {
            if !tmp.is_empty() {
                tmp.sort();
                tmp.reverse();
                mappings.push(tmp);
                tmp = Default::default();
            }
        } else if !line.is_empty() {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            let start = values[1];
            let end = values[1] + values[2] - 1;
            let diff: i64 = values[0] - values[1];
            tmp.push((start, end, diff));
        }
    }

    if !tmp.is_empty() {
        tmp.sort();
        tmp.reverse();
        mappings.push(tmp);
    }
    mappings.reverse();

    let mut location: i64 = 0;
    loop {
        let mut seed: i64 = location;
        for mapping in mappings.iter() {
            seed = unmap_value(mapping, seed);
        }

        location += 1;

        if seeds
            .iter()
            .any(|(start, end)| seed >= *start && seed <= *end)
        {
            break;
        }
    }

    println!("{}", location);
}
