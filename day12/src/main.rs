use std::{collections::HashMap, io};

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

fn count_matches(rec: &Record) -> usize {
    fn inner_(
        springs: &[char],
        counts: &[usize],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if counts.is_empty() {
            // if any damaged, then we've run out of groups
            // otherwise all remaining are ok and we found a possible match
            return if springs.contains(&'#') { 0 } else { 1 };
        }

        if springs.len() < counts.iter().sum::<usize>() + counts.len() {
            // not enough space left for the remaining groups plus
            // their required boundaries (extra . helps here)
            return 0;
        }

        // cache key is the count and spring lengths
        let key = (counts.len(), springs.len());

        if cache.contains_key(&key) {
            // if we've seen this before just return it
            return cache[&key];
        }

        let mut matches = 0;

        if springs[0] != '#' {
            // assume next one is not damaged so skip it
            matches += inner_(&springs[1..], counts, cache);
        }

        let next = counts[0];
        if !springs[..next].contains(&'.') && springs[next] != '#' {
            // consider the next block of springs.
            // none of them are ok, and the spring following the block is
            // not damaged, therefore the next group fits into the
            // next block of springs.
            // extra . helps here too.
            matches += inner_(&springs[next + 1..], &counts[1..], cache);
        }

        // cache and move on, we're done here
        cache.insert(key, matches);
        matches
    }

    // add a . so it's easier to detect the end of the slice
    // without causing a panic by checking beyond bounds
    let mut springs = rec.0.clone();
    springs.push('.');

    // cache with a key of remaining spring length and remaining groups len
    // or (counts.len(), springs.len())
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    inner_(&springs, &rec.1, &mut cache)
}

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Record>) {
    let total: usize = input.iter().map(|rec| count_matches(rec)).sum();
    println!("{}", total);
}

fn part2(input: &Vec<Record>) {
    // replicate each record 5 times with separators
    let input: Vec<Record> = input
        .iter()
        .map(|rec| {
            let chars: Vec<char> = rec
                .0
                .iter()
                .chain([&'?'])
                .cycle()
                .take((rec.0.len() + 1) * 5 - 1)
                .map(|c| *c)
                .collect();
            let counts: Vec<usize> = rec
                .1
                .iter()
                .cycle()
                .take(rec.1.len() * 5)
                .map(|v| *v)
                .collect();

            (chars, counts)
        })
        .collect();
    let total: usize = input.iter().map(|rec| count_matches(rec)).sum();
    println!("{}", total);
}
