fn part1(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut sum = 0;
    for (i, line) in matrix.iter().enumerate() {
        for (j, symbol) in line.iter().enumerate() {
            if !(symbol.is_numeric() || *symbol == '.') {
                for (x, y) in [
                    (1_i32, 0_i32),
                    (0_i32, 1_i32),
                    (-1_i32, 0_i32),
                    (0_i32, -1_i32),
                    (1_i32, 1_i32),
                    (1_i32, -1_i32),
                    (-1_i32, 1_i32),
                    (-1_i32, -1_i32),
                ] {
                    let new_x = ((j as i32) + y) as usize;
                    let new_y = ((i as i32) + x) as usize;
                    let start = matrix.get(new_y).and_then(|line| line.get(new_x));
                    if let Some(c) = start {
                        if visited[new_y][new_x] || !c.is_numeric() {
                            continue;
                        }
                        visited[new_y][new_x] = true;
                        let mut num = String::from(*c);
                        let mut search_x = new_x;
                        while let Some(l) = search_x.checked_sub(1).and_then(|new_search_x| {
                            matrix.get(new_y).and_then(|line| line.get(new_search_x))
                        }) {
                            if !l.is_numeric() {
                                break;
                            }
                            num.insert(0, *l);
                            search_x -= 1;
                            visited[new_y][search_x] = true;
                        }
                        let mut search_x = new_x;
                        while let Some(l) =
                            matrix.get(new_y).and_then(|line| line.get(search_x + 1))
                        {
                            if !l.is_numeric() {
                                break;
                            }
                            num.push(*l);
                            search_x += 1;
                            visited[new_y][search_x] = true;
                        }
                        sum += num.parse::<u32>().unwrap();
                    }
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut sum = 0;
    for (i, line) in matrix.iter().enumerate() {
        for (j, symbol) in line.iter().enumerate() {
            let mut nums = Vec::new();
            if *symbol == '*' {
                for (x, y) in [
                    (1_i32, 0_i32),
                    (0_i32, 1_i32),
                    (-1_i32, 0_i32),
                    (0_i32, -1_i32),
                    (1_i32, 1_i32),
                    (1_i32, -1_i32),
                    (-1_i32, 1_i32),
                    (-1_i32, -1_i32),
                ] {
                    let new_x = ((j as i32) + y) as usize;
                    let new_y = ((i as i32) + x) as usize;
                    let start = matrix.get(new_y).and_then(|line| line.get(new_x));
                    if let Some(c) = start {
                        if visited[new_y][new_x] || !c.is_numeric() {
                            continue;
                        }
                        visited[new_y][new_x] = true;
                        let mut num = String::from(*c);
                        let mut search_x = new_x;
                        while let Some(l) = search_x.checked_sub(1).and_then(|new_search_x| {
                            matrix.get(new_y).and_then(|line| line.get(new_search_x))
                        }) {
                            if !l.is_numeric() {
                                break;
                            }
                            num.insert(0, *l);
                            search_x -= 1;
                            visited[new_y][search_x] = true;
                        }
                        let mut search_x = new_x;
                        while let Some(l) =
                            matrix.get(new_y).and_then(|line| line.get(search_x + 1))
                        {
                            if !l.is_numeric() {
                                break;
                            }
                            num.push(*l);
                            search_x += 1;
                            visited[new_y][search_x] = true;
                        }
                        nums.push(num);
                    }
                }
                if nums.len() == 2 {
                    sum += nums
                        .iter()
                        .map(|num| num.parse::<u32>().unwrap())
                        .product::<u32>();
                }
            }
        }
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Part1: {}\nPart2: {}", part1(&input), part2(&input));
}
