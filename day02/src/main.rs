use regex::Regex;
use std::cmp;
use std::io;

fn main() {
    part2()
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

const GAME_MATCHER: &str = r"(\d+) (red|green|blue)";

#[allow(dead_code)]
fn part1() {
    let matcher: regex::Regex = Regex::new(GAME_MATCHER).unwrap();
    let lines = io::stdin().lines();
    let result = lines.enumerate().fold(0, |acc, (game, line)| -> usize {
        let is_impossible = matcher.captures_iter(&line.unwrap()).any(|capture| {
            let color = &capture[2];
            let value: u32 = capture[1].parse().unwrap();
            let max = match color {
                "red" => MAX_RED,
                "green" => MAX_GREEN,
                "blue" => MAX_BLUE,
                _ => 0,
            };
            value > max
        });

        if is_impossible {
            acc
        } else {
            acc + game + 1
        }
    });

    println!("{}", result);
}

#[allow(dead_code)]
fn part2() {
    let matcher: regex::Regex = Regex::new(GAME_MATCHER).unwrap();
    let lines = io::stdin().lines();
    let result = lines.fold(0, |acc, line| {
        let game = matcher
            .captures_iter(&line.unwrap())
            .fold((0, 0, 0), |acc, capture| {
                let value: u32 = capture[1].parse().unwrap();
                match &capture[2] {
                    "red" => (cmp::max(acc.0, value), acc.1, acc.2),
                    "green" => (acc.0, cmp::max(acc.1, value), acc.2),
                    "blue" => (acc.0, acc.1, cmp::max(acc.2, value)),
                    _ => acc,
                }
            });
        acc + game.0 * game.1 * game.2
    });

    println!("{:?}", result);
}
