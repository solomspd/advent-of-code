use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!("Invalid tile"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Vertical => write!(f, "|"),
            Tile::Horizontal => write!(f, "-"),
            Tile::NorthEast => write!(f, "L"),
            Tile::NorthWest => write!(f, "J"),
            Tile::SouthEast => write!(f, "F"),
            Tile::SouthWest => write!(f, "7"),
            Tile::Ground => write!(f, "."),
            Tile::Start => write!(f, "S"),
        }
    }
}

impl Tile {
    fn traverse(&self, prev_dir: Direction) -> Option<Direction> {
        match self {
            Tile::Vertical => match prev_dir {
                Direction::North => Some(Direction::North),
                Direction::South => Some(Direction::South),
                _ => None,
            },
            Tile::Horizontal => match prev_dir {
                Direction::East => Some(Direction::East),
                Direction::West => Some(Direction::West),
                _ => None,
            },
            Tile::NorthEast => match prev_dir {
                Direction::South => Some(Direction::East),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Tile::NorthWest => match prev_dir {
                Direction::South => Some(Direction::West),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Tile::SouthEast => match prev_dir {
                Direction::North => Some(Direction::East),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Tile::SouthWest => match prev_dir {
                Direction::North => Some(Direction::West),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            Tile::Ground => None,
            Tile::Start => None,
        }
    }

    fn is_bend(&self) -> bool {
        match self {
            Tile::Vertical => false,
            Tile::Horizontal => false,
            Tile::NorthEast => true,
            Tile::NorthWest => true,
            Tile::SouthEast => true,
            Tile::SouthWest => true,
            Tile::Ground => false,
            Tile::Start => false,
        }
    }
}

fn is_turn(prev: Tile, cur: Tile) -> bool {
    match (prev, cur) {
        (Tile::SouthEast, Tile::SouthWest) => true,
        (Tile::NorthEast, Tile::NorthWest) => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

fn find_start_position(map: &Vec<Vec<Tile>>) -> Option<Point> {
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if let Tile::Start = tile {
                return Some(Point { x: j, y: i });
            }
        }
    }
    None
}

fn get_path(map: &Vec<Vec<Tile>>) -> Vec<Point> {
    let start = find_start_position(&map).unwrap();
    let mut path = Vec::new();
    for init_dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    {
        let mut dir = init_dir;
        if start.x == 0 && dir == Direction::West || start.y == 0 && dir == Direction::North {
            continue;
        }
        let mut cur = start + dir;
        path.clear();
        path.push(start);
        while let Some(next_dir) = map[cur.y][cur.x].traverse(dir) {
            path.push(cur);
            let next_point = cur + next_dir;
            if next_point.x >= map[0].len() || next_point.y >= map.len() {
                break;
            }
            if next_point == start {
                return path;
            }
            dir = next_dir;
            cur = next_point;
        }
    }
    unreachable!("No path found")
}

fn part1(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| Tile::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let path = get_path(&map);
    (path.len() as u32 + 1) / 2
}

fn is_enclosed(map: &Vec<Vec<Tile>>, border: &HashSet<Point>, start: Point) -> bool {
    let mut queue = VecDeque::from(vec![start]);
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap().clone();
        visited.insert(cur);
        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if cur.x == 0 && dir == Direction::West || cur.y == 0 && dir == Direction::North {
                return false;
            }
            let new_point = cur + dir;
            if new_point.x >= map[0].len() || new_point.y >= map.len() {
                return false;
            }
            if !visited.contains(&new_point) && !border.contains(&new_point) {
                queue.push_back(new_point);
            }
        }
    }
    if (0..start.x)
        .map(|point| {
            border.contains(&Point {
                x: point,
                y: start.y,
            })
        })
        .filter(|p| *p)
        .count()
        % 2
        == 0
    {
        return false;
    }
    if (0..start.y)
        .map(|point| {
            border.contains(&Point {
                x: start.x,
                y: point,
            })
        })
        .filter(|p| *p)
        .count()
        % 2
        == 0
    {
        return false;
    }
    true
}

fn part2(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| Tile::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let path = get_path(&map);
    let border = path
        .into_iter()
        .enumerate()
        .map(|(index, tile)| (tile, index))
        .collect::<HashMap<_, _>>();
    let mut count = 0;

    for (i, row) in map.iter().enumerate() {
        let mut depth = 0;
        // let mut iter = row.iter().enumerate().peekable();
        let mut j = 0;
        while j < row.len() {
            let cur_point = Point { x: j, y: i };

            if border.contains_key(&cur_point) {
                let start_skip = j;
                while let Some(_) = row.get(j + 1) {
                    let skip_point = Point { x: j, y: i };
                    let next_point = Point { x: j + 1, y: i };

                    match (border.get(&skip_point), border.get(&next_point)) {
                        (Some(skip_order), Some(next_order)) => {
                            if skip_order.abs_diff(*next_order) > 1 {
                                break;
                            }
                        }
                        _ => break,
                    }
                    print!("{}", row[j]);
                    j += 1;
                }
                if !is_turn(row[start_skip], row[j]) {
                    depth += 1;
                }
            }

            if depth % 2 == 1 && !border.contains_key(&cur_point) {
                count += 1;
            }

            j += 1;
        }
    }
    count
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
