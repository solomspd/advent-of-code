use std::collections::VecDeque;

fn part1(input: Vec<Vec<u8>>) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let cur = input[i][j];
            let t = if i == 0 { u8::MAX } else { input[i - 1][j] };
            let r = if j == input[i].len() - 1 { u8::MAX } else { input[i][j + 1] };
            let l = if j == 0 { u8::MAX } else { input[i][j - 1] };
            let b = if i == input.len() - 1 { u8::MAX } else { input[i + 1][j] };
            if t > cur && r > cur && l > cur && b > cur {
                ret += cur as u32 + 1;
            }
        }
    }
    ret
}

fn part2(input: Vec<Vec<u8>>) -> u32 {
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut basins: Vec<u32> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let cur = input[i][j];
            let t = if i == 0 { u8::MAX } else { input[i - 1][j] };
            let r = if j == input[i].len() - 1 { u8::MAX } else { input[i][j + 1] };
            let l = if j == 0 { u8::MAX } else { input[i][j - 1] };
            let b = if i == input.len() - 1 { u8::MAX } else { input[i + 1][j] };
            if t > cur && r > cur && l > cur && b > cur {
                let mut cur_basin: u32 = 0;
                que.push_back((i, j));
                visited[i][j] = true;
                while !que.is_empty() {
                    let cur = que.pop_front().unwrap();
                    cur_basin += 1;
                    for k in [(1,0),(0,1),(-1,0),(0,-1)] {
                        let nx: usize = {
                            let nx = cur.0 as i32 + k.1;
                            if nx < input.len() as i32 && nx >= 0 {
                                nx as usize
                            } else {
                                continue
                            }
                        };
                        let ny = {
                            let ny = cur.1 as i32 + k.0;
                            if ny < input[0].len() as i32 && ny >= 0 {
                                ny as usize
                            } else {
                                continue
                            }
                        };
                        if input[nx][ny] < 9 && !visited[nx][ny] {
                            visited[nx][ny] = true;
                            que.push_back((nx, ny));
                        }
                    }
                }
                basins.push(cur_basin);
            }
        }
    }
    basins.sort();
    basins.iter().rev().take(3).product()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input").lines().map(|l| l.chars().map(|s| s.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
    println!("Part1: {}\nPart2: {}", part1(input.clone()), part2(input.clone()));
}
