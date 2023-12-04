use std::collections::{HashSet, VecDeque};

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let _ = parts
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let mut cards = parts.next().unwrap().split("|");
            let winning = cards
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let pulls = cards
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let intersection = winning.intersection(&pulls).collect::<HashSet<_>>();
            let s = intersection.len() as u32;
            let ret = if s > 0 { (2 as u32).pow(s - 1) } else { 0 };
            ret
        })
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct Card {
    id: usize,
    winning: usize,
}

fn part2(input: &str) -> u32 {
    let card_ref = input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let card_id = parts
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let mut cards = parts.next().unwrap().split("|");
            let winning = cards
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let pulls = cards
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let intersection = winning.intersection(&pulls).collect::<HashSet<_>>();
            Card {
                id: card_id,
                winning: intersection.len(),
            }
        })
        .collect::<Vec<_>>();
    let mut card_num = 0_u32;
    let mut cards = VecDeque::from(card_ref.clone());
    while let Some(card) = cards.pop_back() {
        card_num += 1;
        for new_card in &card_ref[card.id..card.id + card.winning] {
            cards.push_front(new_card.clone());
        }
    }
    card_num
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
