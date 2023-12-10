use std::io;

fn parse_input() -> Vec<Vec<i64>> {
    io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn next_value(_values: &Vec<i64>) -> i64 {
    if _values.iter().all(|v| *v == 0) {
        return 0;
    }

    let mut p = *(_values.first().unwrap());
    let last = *(_values.last().unwrap());
    let next = next_value(
        &_values
            .iter()
            .skip(1)
            .map(|v| {
                let mut v = *v;
                (p, v) = (v, v - p);
                v
            })
            .collect(),
    );
    last + next
}

fn main() {
    let lines = parse_input();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

fn part1(lines: &Vec<Vec<i64>>) -> i64 {
    lines.iter().map(|line| next_value(line)).sum()
}

fn part2(lines: &Vec<Vec<i64>>) -> i64 {
    lines
        .clone()
        .iter_mut()
        .map(|line| {
            line.reverse();
            next_value(line)
        })
        .sum()
}
