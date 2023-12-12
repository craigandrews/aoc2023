use std::{cmp, collections::HashSet, io};

type Coord = (isize, isize);

fn read_input() -> Vec<Vec<char>> {
    io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn find_galaxies(map: &Vec<Vec<char>>) -> Vec<Coord> {
    let mut galaxies: Vec<Coord> = vec![];
    for (y, row) in map.iter().enumerate() {
        let y = y.try_into().unwrap();
        for (x, c) in row.iter().enumerate() {
            let x = x.try_into().unwrap();
            if *c == '#' {
                galaxies.push((x, y));
            }
        }
    }
    galaxies
}

fn empty_rows(map: &Vec<Vec<char>>) -> HashSet<isize> {
    HashSet::from_iter((0..map.len()).filter_map(|row| {
        if map[row].iter().all(|c| *c == '.') {
            Some(row.try_into().unwrap())
        } else {
            None
        }
    }))
}

fn empty_cols(map: &Vec<Vec<char>>) -> HashSet<isize> {
    HashSet::from_iter((0..map[0].len()).filter_map(|col| {
        if map.iter().all(|r| r[col] == '.') {
            Some(col.try_into().unwrap())
        } else {
            None
        }
    }))
}

fn galaxy_pairs(galaxies: &Vec<Coord>) -> Vec<(Coord, Coord)> {
    let mut pairs: Vec<(Coord, Coord)> = vec![];
    for (ix, first) in galaxies.iter().enumerate() {
        for second in galaxies[ix + 1..].iter() {
            pairs.push((*first, *second));
        }
    }
    pairs
}

fn total_distance(expansion: isize, map: &Vec<Vec<char>>) -> isize {
    let expand_cols = empty_cols(&map);
    let expand_rows = empty_rows(&map);
    let galaxies = find_galaxies(&map);

    let mut total = 0;
    for (first, second) in galaxy_pairs(&galaxies) {
        let (sx, ex) = (cmp::min(first.0, second.0), cmp::max(first.0, second.0));
        let (sy, ey) = (cmp::min(first.1, second.1), cmp::max(first.1, second.1));

        total += (sx..ex)
            .map(|x| {
                if expand_cols.contains(&x) {
                    expansion
                } else {
                    1
                }
            })
            .sum::<isize>();
        total += (sy..ey)
            .map(|y| {
                if expand_rows.contains(&y) {
                    expansion
                } else {
                    1
                }
            })
            .sum::<isize>();
    }

    total
}

fn main() {
    let map = read_input();

    part1(&map);
    part2(&map);
}

fn part1(map: &Vec<Vec<char>>) {
    println!("{}", total_distance(2, map));
}

fn part2(map: &Vec<Vec<char>>) {
    println!("{}", total_distance(1000000, map));
}
