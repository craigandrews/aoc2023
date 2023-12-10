use std::{cmp, collections::HashSet, io};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Ground,
    Start,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Ground),
            'S' => Some(Tile::Start),
            '|' => Some(Tile::NS),
            '-' => Some(Tile::EW),
            'L' => Some(Tile::NE),
            'J' => Some(Tile::NW),
            'F' => Some(Tile::SE),
            '7' => Some(Tile::SW),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

type Coord = (usize, usize);

fn parse_input() -> Vec<Vec<Tile>> {
    io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| Tile::from_char(c).unwrap())
                .collect::<Vec<Tile>>()
        })
        .collect()
}

fn find_start(lines: &Vec<Vec<Tile>>) -> Coord {
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == Tile::Start {
                return (y, x);
            }
        }
    }
    (0, 0)
}

fn first_move(start: Coord, lines: &Vec<Vec<Tile>>) -> Option<Dir> {
    if start.0 > 0 {
        let coord: Coord = (start.0 - 1, start.1);
        match lines[coord.0][coord.1] {
            Tile::NS | Tile::SE | Tile::SW => return Some(Dir::N),
            _ => (),
        }
    }

    if start.0 < lines.len() - 1 {
        let coord: Coord = (start.0 + 1, start.1);
        match lines[coord.0][coord.1] {
            Tile::NS | Tile::NE | Tile::NW => return Some(Dir::S),
            _ => (),
        }
    }

    if start.1 > 0 {
        let coord: Coord = (start.0, start.1 - 1);
        match lines[coord.0][coord.1] {
            Tile::EW | Tile::NW | Tile::SW => return Some(Dir::W),
            _ => (),
        }
    }

    if start.1 < lines[start.0].len() - 1 {
        let coord: Coord = (start.0, start.1 + 1);
        match lines[coord.0][coord.1] {
            Tile::EW | Tile::NW | Tile::SW => return Some(Dir::E),
            _ => (),
        }
    }

    None
}

fn main() {
    let lines = parse_input();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

fn part1(lines: &Vec<Vec<Tile>>) -> usize {
    let start = find_start(&lines);
    let mut pos = start;
    let mut dir = first_move(start, &lines).unwrap();

    let mut steps = 0;
    loop {
        steps += 1;

        (pos, dir) = match dir {
            Dir::N => match lines[pos.0 - 1][pos.1] {
                Tile::NS => Some(((pos.0 - 1, pos.1), Dir::N)),
                Tile::SE => Some(((pos.0 - 1, pos.1), Dir::E)),
                Tile::SW => Some(((pos.0 - 1, pos.1), Dir::W)),
                Tile::Start => Some(((pos.0 - 1, pos.1), Dir::N)),
                _ => None,
            },
            Dir::S => match lines[pos.0 + 1][pos.1] {
                Tile::NS => Some(((pos.0 + 1, pos.1), Dir::S)),
                Tile::NE => Some(((pos.0 + 1, pos.1), Dir::E)),
                Tile::NW => Some(((pos.0 + 1, pos.1), Dir::W)),
                Tile::Start => Some(((pos.0 + 1, pos.1), Dir::S)),
                _ => None,
            },
            Dir::E => match lines[pos.0][pos.1 + 1] {
                Tile::EW => Some(((pos.0, pos.1 + 1), Dir::E)),
                Tile::NW => Some(((pos.0, pos.1 + 1), Dir::N)),
                Tile::SW => Some(((pos.0, pos.1 + 1), Dir::S)),
                Tile::Start => Some(((pos.0, pos.1 + 1), Dir::E)),
                _ => None,
            },
            Dir::W => match lines[pos.0][pos.1 - 1] {
                Tile::EW => Some(((pos.0, pos.1 - 1), Dir::W)),
                Tile::NE => Some(((pos.0, pos.1 - 1), Dir::N)),
                Tile::SE => Some(((pos.0, pos.1 - 1), Dir::S)),
                Tile::Start => Some(((pos.0, pos.1 - 1), Dir::W)),
                _ => None,
            },
        }
        .unwrap();

        if lines[pos.0][pos.1] == Tile::Start {
            return steps / 2 + steps % 2;
        }
    }
}

fn part2(lines: &Vec<Vec<Tile>>) -> usize {
    let start = find_start(&lines);
    let mut pos = start;
    let mut dir = first_move(start, &lines).unwrap();

    let mut path_coords: HashSet<Coord> = HashSet::new();
    path_coords.insert(pos);
    loop {
        (pos, dir) = match dir {
            Dir::N => match lines[pos.0 - 1][pos.1] {
                Tile::NS => Some(((pos.0 - 1, pos.1), Dir::N)),
                Tile::SE => Some(((pos.0 - 1, pos.1), Dir::E)),
                Tile::SW => Some(((pos.0 - 1, pos.1), Dir::W)),
                Tile::Start => Some(((pos.0 - 1, pos.1), Dir::N)),
                _ => None,
            },
            Dir::S => match lines[pos.0 + 1][pos.1] {
                Tile::NS => Some(((pos.0 + 1, pos.1), Dir::S)),
                Tile::NE => Some(((pos.0 + 1, pos.1), Dir::E)),
                Tile::NW => Some(((pos.0 + 1, pos.1), Dir::W)),
                Tile::Start => Some(((pos.0 + 1, pos.1), Dir::S)),
                _ => None,
            },
            Dir::E => match lines[pos.0][pos.1 + 1] {
                Tile::EW => Some(((pos.0, pos.1 + 1), Dir::E)),
                Tile::NW => Some(((pos.0, pos.1 + 1), Dir::N)),
                Tile::SW => Some(((pos.0, pos.1 + 1), Dir::S)),
                Tile::Start => Some(((pos.0, pos.1 + 1), Dir::E)),
                _ => None,
            },
            Dir::W => match lines[pos.0][pos.1 - 1] {
                Tile::EW => Some(((pos.0, pos.1 - 1), Dir::W)),
                Tile::NE => Some(((pos.0, pos.1 - 1), Dir::N)),
                Tile::SE => Some(((pos.0, pos.1 - 1), Dir::S)),
                Tile::Start => Some(((pos.0, pos.1 - 1), Dir::W)),
                _ => None,
            },
        }
        .unwrap();

        path_coords.insert(pos);

        if lines[pos.0][pos.1] == Tile::Start {
            break;
        }
    }

    let mut enclosed = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let coord = (y, x);

            if path_coords.contains(&coord) {
                continue;
            }

            let mut n = 0;
            let mut s = 0;
            for i in x + 1..lines[y].len() {
                if !path_coords.contains(&(y, i)) {
                    continue;
                }

                let tile = if lines[y][i] == Tile::Start {
                    Tile::NE // for my input, the S is actually a L
                } else {
                    lines[y][i]
                };

                match tile {
                    Tile::NS => {
                        n += 1;
                        s += 1;
                    }
                    Tile::NW | Tile::NE => n += 1,
                    Tile::SW | Tile::SE => s += 1,
                    Tile::Start => n += 1, // pretend Start = L
                    _ => (),
                }
            }

            enclosed += cmp::min(n, s) % 2;
        }
    }

    enclosed
}
