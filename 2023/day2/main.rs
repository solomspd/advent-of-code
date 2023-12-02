use regex::Regex;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    rounds: Vec<Round>,
}

fn part1(input: &str) -> i32 {
    // (?m)^Game (?P<id>\d+): ((?:((?P<num>\d+) (?P<col>\w+))(?:, )?)+(:?; )?)+
    let re = Regex::new(r"(?m)^Game (?P<id>\d+): ((,?.?(\d+ \w+))+;?)+$").unwrap();
    let _ = re.captures_iter(input).for_each(|line| {
        let id = line["id"].parse::<i32>().unwrap();
        // println!("{:?}", id);
        println!("{:?}", line.get(3));
    });
    todo!()
}

fn part2(input: &str) -> i32 {
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
