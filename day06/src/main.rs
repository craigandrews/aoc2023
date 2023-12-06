use std::io;

fn main() {
    part2();
}

#[allow(dead_code)]
fn part1() {
    let lines: Vec<_> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .into_iter()
                .skip(1)
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let rounds: Vec<(usize, usize)> = lines
        .first()
        .unwrap()
        .iter()
        .zip(lines.last().unwrap().iter())
        .map(|(a, b)| (*a, *b))
        .collect();

    let result: usize = rounds
        .iter()
        .map(|(time, distance)| (1..*time).filter(|t| (*time - t) * t > *distance).count())
        .product();
    println!("{:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let lines: Vec<_> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .into_iter()
                .skip(1)
                .collect::<Vec<_>>()
                .join("")
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    let time = *lines.first().unwrap();
    let distance = *lines.last().unwrap();

    let result = (1..time).filter(|t| (time - t) * t > distance).count();
    println!("{:?}", result);
}
