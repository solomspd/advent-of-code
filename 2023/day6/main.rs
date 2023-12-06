use rayon::prelude::*;

fn part1(input: &str) -> u32 {
    let races = {
        let mut races_split = input.split("\n");

        let times_str = races_split.next().unwrap();
        let distances_str = races_split.next().unwrap();

        times_str
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .zip(
                distances_str
                    .split(":")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap()),
            )
            .collect::<Vec<(u32, u32)>>()
    };
    races
        .par_iter()
        .map(|(time, distance)| {
            (0..=*time)
                .par_bridge()
                .map(|hold| hold * (*time - hold))
                .filter(|new_dist| *new_dist > *distance)
                .count() as u32
        })
        .product()
}

fn part2(input: &str) -> u64 {
    let races = {
        let mut races_split = input.split("\n");

        let times_str = races_split.next().unwrap();
        let distances_str = races_split.next().unwrap();

        vec![(
            times_str
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap(),
            distances_str
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap(),
        )]
    };
    races
        .iter()
        .map(|(time, distance)| {
            (0..=*time)
                .par_bridge()
                .map(|hold| hold * (*time - hold))
                .filter(|new_dist| *new_dist > *distance)
                .count() as u64
        })
        .product()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
