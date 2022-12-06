fn part1(input: &String) -> u32 {
	let mut arr: [i32;12] = [0;12];
	for i in input.lines() {
		for (j, jj) in i.chars().enumerate() {
			match jj {
				'1' => arr[j] += 1,
				'0' => arr[j] -= 1,
				_ => panic!("Invalid input"),
			}
		}
	}
	let mut gamma:u32 = 0;
	for (i, &ii) in arr.iter().rev().enumerate() {
		if ii > 0 {
			gamma |= 1 << i;
		} else if ii < 0 {
			gamma &= !(1 << i);
		}
	}
	let epsilon:u32 = (!gamma) & !(!0 << arr.len());
	gamma * epsilon
}

fn part2(input: &String) -> u32 {
	let input: Vec<&str> = input.lines().collect();
	let mut consider = input.clone();
	let mut i:usize = 0;
	while consider.len() > 1 {
		let mut count = 0;
		for j in consider.iter() {
			match j.chars().nth(i).unwrap() {
				'1' => count += 1,
				'0' => count -= 1,
				_ => panic!("Invalid input"),
			}
		}
		if count >= 0 {
			consider.retain(|&x| x.chars().nth(i).unwrap() == '1');
		} else {
			consider.retain(|&x| x.chars().nth(i).unwrap() == '0');
		}
		i += 1;
	}
	let o2 = u32::from_str_radix(consider[0], 2).unwrap();
	consider = input.clone();
	i = 0;
	while consider.len() > 1 {
		let mut count = 0;
		for j in consider.iter() {
			match j.chars().nth(i).unwrap() {
				'1' => count += 1,
				'0' => count -= 1,
				_ => panic!("Invalid input"),
			}
		}
		if count < 0 {
			consider.retain(|&x| x.chars().nth(i).unwrap() == '1');
		} else {
			consider.retain(|&x| x.chars().nth(i).unwrap() == '0');
		}
		i += 1;
	}
	let co2 = u32::from_str_radix(consider[0], 2).unwrap();

	o2 * co2
}

fn main() {
	let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
	println!("{}\n{}", part1(&input), part2(&input));
}