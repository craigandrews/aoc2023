use std::collections::HashSet;
use std::io;

fn main() {
    let lines = io::stdin().lines().map(|line| {
        line.unwrap()
            .splitn(2, ":")
            .nth(1)
            .unwrap()
            .trim()
            .to_owned()
    });

    let mut sum: u32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.splitn(2, "|").map(|s| s.trim()).collect();

        let winning: HashSet<u32> = parts[0]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let have: HashSet<u32> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let num_winners: u32 = winning.intersection(&have).count().try_into().unwrap();

        if num_winners > 0 {
            sum += 2_u32.pow(num_winners - 1);
        }
    }

    println!("{}", sum);
}
