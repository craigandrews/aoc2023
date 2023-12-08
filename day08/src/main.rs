use regex::Regex;
use std::{collections::HashMap, io};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, &'static str> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err("invalid"),
        }
    }
}

fn main() {
    part2();
}

#[allow(dead_code)]
fn part1() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap());
    let re = Regex::new(r"[A-Z]{3}").unwrap();

    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Direction::from_char(c).unwrap())
        .collect();
    lines.next();

    let locations: HashMap<String, (String, String)> = HashMap::from_iter(lines.map(|line| {
        let matches: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();
        (
            matches[0].to_owned(),
            (matches[1].to_owned(), matches[2].to_owned()),
        )
    }));

    let mut loc: String = "AAA".to_owned();
    let mut steps = 0;
    for dir in directions.iter().cycle() {
        steps += 1;
        let paths = &locations[&loc];

        let next_loc = match dir {
            Direction::Left => &paths.0,
            Direction::Right => &paths.1,
        };
        loc = next_loc.to_string();
        if loc == "ZZZ" {
            break;
        }
    }
    println!("{}", steps);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b)
    }
    a
}

#[allow(dead_code)]
fn part2() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap());
    let re = Regex::new(r"[A-Z0-9]{3}").unwrap();

    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Direction::from_char(c).unwrap())
        .collect();
    lines.next();

    let locations: HashMap<String, (String, String)> = HashMap::from_iter(lines.map(|line| {
        let matches: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();
        (
            matches[0].to_owned(),
            (matches[1].to_owned(), matches[2].to_owned()),
        )
    }));

    let curr_pos: Vec<String> = locations
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.clone())
        .collect();

    let max_steps: Vec<u64> = curr_pos
        .iter()
        .map(|loc| {
            let mut loc = loc;
            let mut steps = 0;
            for dir in directions.iter().cycle() {
                steps += 1;
                let paths = &locations[loc];

                let next_loc = match dir {
                    Direction::Left => &paths.0,
                    Direction::Right => &paths.1,
                };
                loc = next_loc;
                if loc.ends_with("Z") {
                    break;
                }
            }
            steps
        })
        .collect();

    let mut result: u64 = max_steps[0];
    for x in max_steps {
        result = (result * x) / gcd(result, x);
    }

    println!("{}", result);
}
