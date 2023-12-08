use std::collections::HashMap;

use itertools::Itertools;
use num::integer::lcm;
use rayon::prelude::*;

enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let mut split = input.lines();
    (
        split
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!("Invalid direction"),
            })
            .collect::<Vec<_>>(),
        split
            .skip(1)
            .map(|line| {
                let mut split = line.split("=");
                let src = split.next().unwrap().trim();
                let binding = split.next().unwrap().replace(&['(', ')'][..], "");
                let (l, r) = binding
                    .split(",")
                    .map(|dir| dir.trim())
                    .collect_tuple()
                    .unwrap();
                (src.to_owned(), (l.to_owned(), r.to_owned()))
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn part1(input: &str) -> u32 {
    let (dirs, map) = parse_input(input);
    let mut cur = "AAA";
    let mut dirs = dirs.iter().cycle();
    let mut steps = 0;
    loop {
        steps += 1;
        let (l, r) = map.get(cur).unwrap();
        let dir = dirs.next().unwrap();
        cur = match dir {
            Direction::Left => l,
            Direction::Right => r,
        };
        if cur == "ZZZ" {
            break;
        }
    }
    steps
}

fn part2(input: &str) -> u64 {
    let (dirs, map) = parse_input(input);
    let cur = map.keys().filter(|k| k.ends_with("A")).collect::<Vec<_>>();
    let routes = cur.par_iter().map(|start| {
        let mut dirs = dirs.iter().cycle();
        let mut c = *start;
        let mut steps = 0;
        loop {
            steps += 1;
            let (l, r) = map.get(c).unwrap();
            let dir = dirs.next().unwrap();
            c = match dir {
                Direction::Left => l,
                Direction::Right => r,
            };
            if c.ends_with("Z") {
                break;
            }
        }
        steps
    });
    routes
        .fold(|| 1, |acc, x| lcm(acc, x))
        .reduce(|| 1, |acc, x| lcm(acc, x))
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
