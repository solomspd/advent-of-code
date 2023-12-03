use std::cmp::max;
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Add for Round {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sum for Round {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Self {
                red: 0,
                green: 0,
                blue: 0,
            },
            |a, b| a + b,
        )
    }
}

// FIXME: borked
impl std::cmp::PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(std::cmp::Ordering::Equal)
        } else if self.red < other.red || self.green < other.green || self.blue < other.blue {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

const BAG: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut s = line.split(":");
            let game_id = s.next().unwrap().split(" ").skip(1).next().unwrap();
            let rounds = s
                .next()
                .unwrap()
                .split(";")
                .map(|round| {
                    let mut r: u32 = 0;
                    let mut g: u32 = 0;
                    let mut b: u32 = 0;
                    for (count, color) in round.split(",").map(|x| {
                        let mut iter = x.split_whitespace();
                        (iter.next().unwrap(), iter.next().unwrap())
                    }) {
                        let count = count.trim().parse().unwrap();
                        match color.trim() {
                            "red" => r = count,
                            "green" => g = count,
                            "blue" => b = count,
                            _ => panic!("invalid color"),
                        }
                    }
                    Round {
                        red: r,
                        green: g,
                        blue: b,
                    }
                })
                .collect::<Vec<_>>();
            Game {
                id: game_id.parse::<u32>().unwrap(),
                rounds: rounds,
            }
        })
        .filter(|game| {
            game.rounds
                .iter()
                .map(|round| {
                    round.red <= BAG.red && round.green <= BAG.green && round.blue <= BAG.blue
                })
                .all(|x| x)
        })
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut s = line.split(":");
            let game_id = s.next().unwrap().split(" ").skip(1).next().unwrap();
            let rounds = s
                .next()
                .unwrap()
                .split(";")
                .map(|round| {
                    let mut r: u32 = 0;
                    let mut g: u32 = 0;
                    let mut b: u32 = 0;
                    for (count, color) in round.split(",").map(|x| {
                        let mut iter = x.split_whitespace();
                        (iter.next().unwrap(), iter.next().unwrap())
                    }) {
                        let count = count.trim().parse().unwrap();
                        match color.trim() {
                            "red" => r = count,
                            "green" => g = count,
                            "blue" => b = count,
                            _ => panic!("invalid color"),
                        }
                    }
                    Round {
                        red: r,
                        green: g,
                        blue: b,
                    }
                })
                .collect::<Vec<_>>();
            Game {
                id: game_id.parse::<u32>().unwrap(),
                rounds: rounds,
            }
        })
        .map(|game| {
            game.rounds
                .into_iter()
                .reduce(|acc, e| Round {
                    red: max(acc.red, e.red),
                    green: max(acc.green, e.green),
                    blue: max(acc.blue, e.blue),
                })
                .unwrap()
                .power()
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
