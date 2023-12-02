use regex::Regex;
use std::cmp;
use std::io::{self, BufRead};

fn main() {
    part2()
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[allow(dead_code)]
fn part1() {
    let matcher: regex::Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let stdin = io::stdin();
    let mut game: u32 = 0;
    println!(
        "{}",
        stdin
            .lock()
            .lines()
            .map(|line| -> u32 {
                game += 1;
                if matcher.captures_iter(&line.unwrap()).any(|capture| match &capture[2] {
                        "red" => MAX_RED,
                        "green" => MAX_GREEN,
                        "blue" => MAX_BLUE,
                        _ => 0,
                    } <= capture[1].parse().unwrap()
                ) { 0 } else { game }
            })
            .sum::<u32>()
    );
}

#[allow(dead_code)]
fn part2() {
    let matcher: regex::Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let stdin = io::stdin();

    println!(
        "{:?}",
        stdin
            .lock()
            .lines()
            .map(|line| matcher
                .captures_iter(&line.unwrap())
                .fold((0, 0, 0), |acc, capture| {
                    let value: u32 = capture[1].parse().unwrap();
                    match &capture[2] {
                        "red" => (cmp::max(acc.0, value), acc.1, acc.2),
                        "green" => (acc.0, cmp::max(acc.1, value), acc.2),
                        "blue" => (acc.0, acc.1, cmp::max(acc.2, value)),
                        _ => acc,
                    }
                }))
            .map(|game| game.0 * game.1 * game.2)
            .sum::<u32>()
    );
}
