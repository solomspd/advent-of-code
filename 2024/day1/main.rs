use std::collections::HashMap;

fn part1(input: &str) -> u32 {
	let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = input
		.lines()
		.map(|line| {
			let mut parts = line.split_whitespace();
			let part1 = parts.next().unwrap().parse::<u32>().unwrap();
			let part2 = parts.next().unwrap().parse::<u32>().unwrap();
			(part1, part2)
		})
		.unzip();
	col1.sort();
	col2.sort();
	col1.iter().zip(col2.iter())
		.map(|(x, y)| (x).abs_diff(*y))
		.sum()
}

fn part2(input: &str) -> u32 {
	let (col1, col2): (Vec<u32>, Vec<u32>) = input
		.lines()
		.map(|line| {
			let mut parts = line.split_whitespace();
			let part1 = parts.next().unwrap().parse::<u32>().unwrap();
			let part2 = parts.next().unwrap().parse::<u32>().unwrap();
			(part1, part2)
		})
		.unzip();

	let mut counts1 = HashMap::new();
	for &value in col1.iter() {
		*counts1.entry(value).or_insert(0) += 1;
	}
	let mut counts2 = HashMap::new();
	for &value in col2.iter() {
		*counts2.entry(value).or_insert(0) += 1;
	}
	
	let mut sum = 0;
	for (&key, &count1) in counts1.iter() {
		if let Some(&count2) = counts2.get(&key) {
			sum += key * count2 * count1;
		}
	}
	sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}