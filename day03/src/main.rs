use std::cmp;
use std::io;

fn main() {
    part1();
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn find_digit_groups(input: &Vec<char>) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut current_number = String::new();
    let mut current_index = None;

    for (index, character) in input.iter().enumerate() {
        if character.is_numeric() {
            current_number.push(*character);
            if current_index.is_none() {
                current_index = Some(index);
            }
        } else if !current_number.is_empty() {
            result.push((current_index.unwrap(), current_number.clone()));
            current_number.clear();
            current_index = None;
        }
    }

    if !current_number.is_empty() {
        result.push((current_index.unwrap(), current_number));
    }

    result
}

fn part1() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut sum: u32 = 0;
    for (nr, digit_group) in lines.iter().map(|l| find_digit_groups(&l)).enumerate() {
        for (pos, digits) in digit_group.iter() {
            let first_line = if nr > 0 { nr - 1 } else { 0 };
            let last_line = cmp::min(lines.len() - 1, nr + 1);
            if lines[first_line..last_line + 1].iter().any(|l| {
                let start = if *pos > 0 { pos - 1 } else { 0 };
                let end = cmp::min(l.len() - 1, pos + digits.len());
                l[start..end + 1].iter().any(|c| is_symbol(*c))
            }) {
                sum += digits.parse::<u32>().unwrap();
            }
        }
    }
    println!("{}", sum);
}
