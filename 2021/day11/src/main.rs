use std::collections::VecDeque;

fn part1(mut input: Vec<Vec<u8>>, steps: Option<u128>) -> u128 {
    let steps = steps.unwrap_or(100);
    let mut flashes = 0;
    for _ in 0..steps {
        let mut flashed = vec![vec![false; input.len()]; input[0].len()];
        let mut que: VecDeque<(usize,usize)> = VecDeque::new();
        for (i,ii) in input.iter_mut().enumerate() {
            for (j,jj) in ii.iter_mut().enumerate() {
                *jj += 1;
                if *jj > 9 {
                    que.push_back((i,j));
                }
            }
        }
        while !que.is_empty() {
            let cur = que.pop_front().unwrap();
            input[cur.0][cur.1] = 0;
            flashed[cur.0][cur.1] = true;
            flashes += 1;
            for k in -1..=1 as i32 {
                for l in -1..=1 as i32 {
                    if k == 0 && l == 0 { continue; }
                    let nx = {
                        let nx = cur.0 as i32 + k;
                        if nx < 0 || nx >= input.len() as i32 { continue; }
                        nx as usize
                    };
                    let ny = {
                        let ny = cur.1 as i32 + l;
                        if ny < 0 || ny >= input[cur.1].len() as i32 { continue; }
                        ny as usize
                    };
                    if !flashed[nx][ny] {
                        input[nx][ny] += 1;
                        if input[nx][ny] > 9 && !que.contains(&(nx,ny)) {
                            que.push_back((nx,ny));
                        }
                    }
                }
            }
        }
    }
    flashes
}

fn part2(mut input: Vec<Vec<u8>>) -> u128 {
    let mut steps = 0;
    while input.iter().map(|x| x.iter().map(|&x| x as u128).sum::<u128>()).sum::<u128>() > 0 {
        let mut flashed = vec![vec![false; input.len()]; input[0].len()];
        let mut que: VecDeque<(usize,usize)> = VecDeque::new();
        for (i,ii) in input.iter_mut().enumerate() {
            for (j,jj) in ii.iter_mut().enumerate() {
                *jj += 1;
                if *jj > 9 {
                    que.push_back((i,j));
                }
            }
        }
        while !que.is_empty() {
            let cur = que.pop_front().unwrap();
            input[cur.0][cur.1] = 0;
            flashed[cur.0][cur.1] = true;
            for k in -1..=1 as i32 {
                for l in -1..=1 as i32 {
                    if k == 0 && l == 0 { continue; }
                    let nx = {
                        let nx = cur.0 as i32 + k;
                        if nx < 0 || nx >= input.len() as i32 { continue; }
                        nx as usize
                    };
                    let ny = {
                        let ny = cur.1 as i32 + l;
                        if ny < 0 || ny >= input[cur.1].len() as i32 { continue; }
                        ny as usize
                    };
                    if !flashed[nx][ny] {
                        input[nx][ny] += 1;
                        if input[nx][ny] > 9 && !que.contains(&(nx,ny)) {
                            que.push_back((nx,ny));
                        }
                    }
                }
            }
        }
        steps += 1;
    }
    steps
}

fn main() {
    let input:Vec<Vec<u8>> = std::fs::read_to_string("tst_input.txt").expect("Failed to read input").lines().map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as u8).collect()).collect();
    println!("Part1: {}\nPart2: {}", part1(input.clone(), None), part2(input.clone()));
}
