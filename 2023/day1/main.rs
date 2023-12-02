use lazy_static::lazy_static;
use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let ret: i32 = lines
        .map(|line| {
            let nums = line
                .chars()
                .filter(|x| x.is_numeric())
                .collect::<Vec<char>>();
            let ret = format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
                .parse::<i32>()
                .unwrap();
            ret
        })
        .sum();
    ret
}

lazy_static! {
    static ref LUT: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert("one", '1');
        m.insert("two", '2');
        m.insert("three", '3');
        m.insert("four", '4');
        m.insert("five", '5');
        m.insert("six", '6');
        m.insert("seven", '7');
        m.insert("eight", '8');
        m.insert("nine", '9');
        m
    };
}

fn part2(input: &str) -> i32 {
    let lines = input.lines();
    let ret: i32 = lines
        .map(|line| {
            let mut nums: Vec<(usize, char)> = Vec::new();
            LUT.iter().for_each(|(key, value)| {
                line.match_indices(key)
                    .for_each(|(index, _)| nums.push((index, *value)));
            });
            nums.extend(
                line.chars()
                    .enumerate()
                    .filter(|(_, x)| x.is_numeric())
                    .collect::<Vec<(usize, char)>>(),
            );
            format!(
                "{}{}",
                nums.iter()
                    .min_by(|(x, _), (y, _)| x.cmp(y))
                    .map(|(_, x)| x)
                    .unwrap(),
                nums.iter()
                    .max_by(|(x, _), (y, _)| x.cmp(y))
                    .map(|(_, x)| x)
                    .unwrap(),
            )
            .parse::<i32>()
            .unwrap()
        })
        .sum();
    ret
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
