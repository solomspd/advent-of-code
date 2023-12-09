fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut series = vec![nums.clone()];
            while !nums.iter().all(|x| *x == 0) {
                nums = nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
                series.push(nums.clone())
            }
            series.reverse();
            series
                .iter()
                .skip(1)
                .fold(0, |acc, x| acc + x.last().unwrap())
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut series = vec![nums.clone()];
            while !nums.iter().all(|x| *x == 0) {
                nums = nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
                series.push(nums.clone())
            }
            series.reverse();
            series
                .iter()
                .skip(1)
                .fold(0, |acc, x| x.first().unwrap() - acc)
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
