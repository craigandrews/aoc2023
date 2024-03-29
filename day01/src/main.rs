use std::io;

fn main() {
    part2()
}

#[allow(dead_code)]
fn part1() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    for line in stdin.lines() {
        let line = line.expect("Failed to read line");

        let first_digit = line
            .chars()
            .find(|c| c.is_numeric())
            .expect("no digit found")
            .to_digit(10)
            .expect("failed to parse");

        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .expect("no digit found")
            .to_digit(10)
            .expect("failed to parse");

        sum += (first_digit * 10) + last_digit;
    }

    println!("{:?}", sum);
}

const DIGITS: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

#[allow(dead_code)]
fn part2() {
    println!(
        "{}",
        io::stdin()
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();

                let first = DIGITS
                    .into_iter()
                    .filter_map(|(substring, value)| line.find(substring).map(|pos| (pos, value)))
                    .min_by_key(|&(pos, _)| pos);

                let last = DIGITS
                    .into_iter()
                    .filter_map(|(substring, value)| line.rfind(substring).map(|pos| (pos, value)))
                    .max_by_key(|&(pos, _)| pos);

                match (first, last) {
                    (Some(first), Some(last)) => Some(first.1 * 10 + last.1),
                    _ => None,
                }
            })
            .sum::<u32>()
    );
}
