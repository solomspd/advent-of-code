use itertools::Itertools;

type GalaxyId = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Space,
    Galaxy(GalaxyId),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Delta {
    x: isize,
    y: isize,
}

impl Delta {
    fn mag(self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn sub(self, other: Self) -> Delta {
        Delta {
            x: self.x as isize - other.x as isize,
            y: self.y as isize - other.y as isize,
        }
    }
}

fn expand_universe(input: &str, factor: usize) -> Vec<Loc> {
    let factor = factor - 1;
    let mut id = 1;
    let compacted = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let tile = match c {
                        '.' => Tile::Space,
                        '#' => {
                            let tile = Tile::Galaxy(id);
                            id += 1;
                            tile
                        }
                        _ => unreachable!("Invalid input"),
                    };
                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let locs = compacted
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, tile)| match tile {
                    Tile::Galaxy(_) => Some(Loc { x: j, y: i }),
                    _ => None,
                })
        })
        .collect::<Vec<_>>();

    let row_expand = compacted
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|tile| *tile == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let width = compacted[0].len();
    let col_expand = (0..width)
        .filter_map(|i| {
            if compacted.iter().all(|row| row[i] == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    locs.iter()
        .map(|loc| Loc {
            x: loc.x + (factor * col_expand.binary_search(&loc.x).unwrap_err()),
            y: loc.y + (factor * row_expand.binary_search(&loc.y).unwrap_err()),
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u64 {
    let galaxies = expand_universe(input, 2);
    let pairs = galaxies
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]));
    pairs.map(|(src, dst)| src.sub(*dst).mag()).sum()
}

fn part2(input: &str) -> u64 {
    let galaxies = expand_universe(input, 1000000);
    let pairs = galaxies
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]));
    pairs.map(|(src, dst)| src.sub(*dst).mag()).sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
