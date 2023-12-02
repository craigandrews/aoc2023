use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    part1()
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
                if matcher.captures_iter(&line.unwrap()).any(|capture| {
                    let value: u32 = capture[1].parse().unwrap();
                    let max = match &capture[2] {
                        "red" => MAX_RED,
                        "green" => MAX_GREEN,
                        "blue" => MAX_BLUE,
                        _ => 0,
                    };
                    return value > max;
                }) {
                    return 0;
                }
                return game;
            })
            .sum::<u32>()
    );
}
