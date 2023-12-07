use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;

lazy_static! {
    static ref CHAR_TO_CARD: HashMap<char, Card> = {
        let mut map = HashMap::new();
        map.insert('A', Card::A);
        map.insert('K', Card::K);
        map.insert('Q', Card::Q);
        map.insert('J', Card::J);
        map.insert('T', Card::T);
        map.insert('9', Card::Nine);
        map.insert('8', Card::Eight);
        map.insert('7', Card::Seven);
        map.insert('6', Card::Six);
        map.insert('5', Card::Five);
        map.insert('4', Card::Four);
        map.insert('3', Card::Three);
        map.insert('2', Card::Two);
        map
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Types {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
enum Card {
    J2,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    hand: [Card; 5],
    bet: u32,
    type_: Types,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.type_ == other.type_ {
            let mut res = Ordering::Equal;
            for (s, o) in self.hand.iter().zip(other.hand.iter()) {
                if s != o {
                    res = s.cmp(o);
                    break;
                }
            }
            Some(res)
        } else {
            Some(self.type_.cmp(&other.type_))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn new(hand: &[char], bet: u32) -> Self {
        let hand = {
            let hand_vec = hand
                .iter()
                .map(|c| *CHAR_TO_CARD.get(c).unwrap())
                .collect::<Vec<_>>();
            [
                hand_vec[0],
                hand_vec[1],
                hand_vec[2],
                hand_vec[3],
                hand_vec[4],
            ]
        };
        let type_ = {
            let mut count = [0; 5];
            let sorted = {
                let mut sorted = hand.clone();
                sorted.sort();
                sorted
            };
            let mut i = 0;
            let mut count_index = 0;
            let mut cur = sorted[0];
            while i < 5 {
                count[count_index] += 1;
                i += 1;
                while i < 5 && cur == sorted[i] {
                    count[count_index] += 1;
                    i += 1;
                }
                if i == 5 {
                    break;
                }
                cur = sorted[i];
                count_index += 1;
            }
            count.sort();
            count.reverse();
            match count {
                [5, 0, 0, 0, 0] => Types::FiveOfAKind,
                [4, 1, 0, 0, 0] => Types::FourOfAKind,
                [3, 2, 0, 0, 0] => Types::FullHouse,
                [3, 1, 1, 0, 0] => Types::ThreeOfAKind,
                [2, 2, 1, 0, 0] => Types::TwoPair,
                [2, 1, 1, 1, 0] => Types::OnePair,
                [1, 1, 1, 1, 1] => Types::HighCard,
                _ => panic!("this shouldnt be happening"),
            }
        };
        println!("{:?}::{:?}", hand, type_);
        Self { hand, bet, type_ }
    }
}

impl Hand {
    fn newp2(hand: &[char], bet: u32) -> Self {
        let hand = {
            let hand_vec = hand
                .iter()
                .map(|c| *CHAR_TO_CARD.get(c).unwrap())
                .map(|c| match c {
                    Card::J => Card::J2,
                    _ => c,
                })
                .collect::<Vec<_>>();
            [
                hand_vec[0],
                hand_vec[1],
                hand_vec[2],
                hand_vec[3],
                hand_vec[4],
            ]
        };
        let type_ = {
            let mut count = [0; 5];
            let sorted = {
                let mut sorted = hand.clone();
                sorted.sort();
                sorted
            };
            let mut i = 0;
            let mut count_index = 0;
            let mut js = 0;
            while i < 5 {
                let cur = sorted[i];
                i += 1;
                if cur == Card::J2 {
                    js += 1;
                    continue;
                }
                count[count_index] += 1;
                while i < 5 && cur == sorted[i] {
                    count[count_index] += 1;
                    i += 1;
                }
                if i == 5 {
                    break;
                }
                count_index += 1;
            }
            count.sort();
            count.reverse();
            count[0] += js;
            match count {
                [5, 0, 0, 0, 0] => Types::FiveOfAKind,
                [4, 1, 0, 0, 0] => Types::FourOfAKind,
                [3, 2, 0, 0, 0] => Types::FullHouse,
                [3, 1, 1, 0, 0] => Types::ThreeOfAKind,
                [2, 2, 1, 0, 0] => Types::TwoPair,
                [2, 1, 1, 1, 0] => Types::OnePair,
                [1, 1, 1, 1, 1] => Types::HighCard,
                _ => panic!("this shouldnt be happening"),
            }
        };
        println!("{:?}::{:?}", hand, type_);
        Self { hand, bet, type_ }
    }
}

fn part1(input: &str) -> u32 {
    let hands = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            Hand::new(
                &split.next().unwrap().chars().collect::<Vec<char>>(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(rank, Hand { bet, .. })| (rank as u32 + 1) * *bet)
        .sum()
}

fn part2(input: &str) -> u32 {
    let hands = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            Hand::newp2(
                &split.next().unwrap().chars().collect::<Vec<char>>(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(rank, Hand { bet, .. })| (rank as u32 + 1) * *bet)
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
