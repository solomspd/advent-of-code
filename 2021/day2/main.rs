fn part1(input: &String) -> i32 {
	let mut xcord = 0;
	let mut ycord = 0;
	for i in input.lines() {
		let mut words = i.split_whitespace();
		let dir = words.next().unwrap();
		let val = words.next().unwrap().parse::<i32>().unwrap();
		match dir {
			"forward" => xcord += val,
			"down" => ycord += val,
			"up" => ycord -= val,
			_ => panic!("Unknown direction"),
		}
	}
	xcord * ycord
}

fn part2(input: &String) -> i32 {
	let mut xcord = 0;
	let mut ycord = 0;
	let mut aim = 0;
	for i in input.lines() {
		let mut words = i.split_whitespace();
		let dir = words.next().unwrap();
		let val = words.next().unwrap().parse::<i32>().unwrap();
		match dir {
			"forward" => {xcord += val; ycord += aim*val},
			"down" => aim += val,
			"up" => aim -= val,
			_ => panic!("Unknown direction"),
		}
	}
	xcord * ycord
}

fn main() {
	let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
	println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}