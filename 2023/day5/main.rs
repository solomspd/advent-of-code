use itertools::Itertools;
use rangemap::RangeMap;
use rayon::prelude::*;

fn part1(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect::<Vec<i64>>();

    let mappings = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let (dst_rng_start, src_rng_start, interval) = line
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (
                        src_rng_start..src_rng_start + interval,
                        dst_rng_start - src_rng_start,
                    )
                })
                .collect::<RangeMap<i64, i64>>()
        })
        .collect::<Vec<_>>();
    seeds
        .iter()
        .map(|seed| {
            mappings.iter().fold(*seed, |seed, mapping| {
                seed + *mapping.get(&seed).unwrap_or(&0)
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .flat_map(|chunk| {
            let (start, interval) = chunk.iter().cloned().collect_tuple().unwrap();
            start..start + interval
        })
        .collect::<Vec<i64>>();

    let mappings = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let (dst_rng_start, src_rng_start, interval) = line
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (
                        src_rng_start..src_rng_start + interval,
                        dst_rng_start - src_rng_start,
                    )
                })
                .collect::<RangeMap<i64, i64>>()
        })
        .collect::<Vec<_>>();
    seeds
        .par_iter()
        .map(|seed| {
            mappings.iter().fold(*seed, |seed, mapping| {
                seed + *mapping.get(&seed).unwrap_or(&0)
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
