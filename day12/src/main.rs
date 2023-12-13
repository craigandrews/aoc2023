use itertools::Itertools;
use std::io;

type Counts = Vec<usize>;

type Record = (Vec<char>, Counts);

fn read_input() -> Vec<Record> {
    io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (record, p) = line.split_once(" ").unwrap();
            let pattern = p.split(",").map(|v| v.parse().unwrap()).collect();
            (record.chars().collect(), pattern)
        })
        .collect()
}

fn is_valid(rec: &Record) -> bool {
    rec.0
        .iter()
        .group_by(|c| *c)
        .into_iter()
        .filter_map(|(k, g)| match k {
            '#' => Some(g.count()),
            _ => None,
        })
        .eq(rec.1.iter().copied())
}

fn count_matches(rec: &Record) -> usize {
    let pos = rec.0.iter().position(|c| *c == '?');
    match pos {
        Some(pos) => {
            let mut as_empty = rec.0.clone();
            as_empty[pos] = '.';

            let mut as_spring = rec.0.clone();
            as_spring[pos] = '#';

            let t = count_matches(&(as_empty, rec.1.clone()))
                + count_matches(&(as_spring, rec.1.clone()));
            println!("{}", t);
            t
        }
        None => {
            if is_valid(rec) {
                1
            } else {
                0
            }
        }
    }
}

fn main() {
    let input = read_input();
    part1(&input);
}

fn part1(input: &Vec<Record>) {
    let total: usize = input.iter().map(|rec| count_matches(rec)).sum();
    println!("{}", total);
}
