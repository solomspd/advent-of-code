fn part1(input: &String) -> u32 {
	let mut lns = input.split("\n\n");
	let draw:Vec<u32> = lns.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
	let mut boards:Vec<Vec<Vec<(u32,bool)>>> = lns.map(|x| x.lines().map(|z| z.split_whitespace().map(|y| (y.parse::<u32>().unwrap(),false)).collect()).collect()).collect();
	for i in draw {
		for j in boards.iter_mut() {
			for k in j.iter_mut() {
				for l in k.iter_mut() {
					if l.0 == i {
						l.1 = true;
					}
				}
			}

			let mut hwin = true;
			for k in j.iter_mut() {
				hwin = true;
				for l in k.iter_mut() {
					if l.1 == false {
						hwin = false;
					}
				}
				if hwin {
					break;
				}
			}

			let mut vwin = true;
			for (k,kk) in j.iter().enumerate() {
				vwin = true;
				for l in 0..kk.len() {
					if j[l][k].1 == false {
						vwin = false;
					}
				}
				if vwin {
					break;
				}
			}

			if vwin || hwin {
				let mut sum  = 0;
				for k in j {
					for l in k {
						if !l.1 {
							sum += l.0;
						}
					}
				}
				return i * sum;
			}
		}
	}
	0
}

fn part2(input: &String) -> u32 {
	let mut lns = input.split("\n\n");
	let draw:Vec<u32> = lns.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
	let mut boards:Vec<Vec<Vec<(u32,bool)>>> = lns.map(|x| x.lines().map(|z| z.split_whitespace().map(|y| (y.parse::<u32>().unwrap(),false)).collect()).collect()).collect();
	let mut n_boards = boards.len();
	for i in draw {
		let mut keep = vec![true; boards.len()];
		for (jj,j) in boards.iter_mut().enumerate() {
			for k in j.iter_mut() {
				for l in k.iter_mut() {
					if l.0 == i {
						l.1 = true;
					}
				}
			}

			let mut hwin = true;
			for k in j.iter_mut() {
				hwin = true;
				for l in k.iter_mut() {
					if l.1 == false {
						hwin = false;
					}
				}
				if hwin {
					break;
				}
			}

			let mut vwin = true;
			for (k,kk) in j.iter().enumerate() {
				vwin = true;
				for l in 0..kk.len() {
					if j[l][k].1 == false {
						vwin = false;
					}
				}
				if vwin {
					break;
				}
			}

			if vwin || hwin {
				if n_boards == 1 {
					let mut sum  = 0;
					for k in j {
						for l in k {
							if !l.1 {
								sum += l.0;
							}
						}
					}
					return i * sum;
				} else {
					keep[jj] = false;
					n_boards -= 1;
				}
			}
		}
		let mut iter = keep.iter();
		boards.retain(|_| *iter.next().unwrap());
	}
	0
}

fn main() {
	let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
	println!("{}", part2(&input));
}