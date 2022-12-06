use std::collections::HashMap;

fn part1(input: Vec<&str>) -> u32 {
    let mut ret = 0;
    let pair = HashMap::from([('<','>'),('{','}'),('[',']'),('(',')')]);
    for i in input {
        let mut brk: Vec<char> = Vec::new();
        for j in i.chars() {
            if pair.contains_key(&j) {
                brk.push(j);
            } else {
                let last = brk.pop().unwrap();
                if pair[&last] != j {
                    ret += match j {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("Invalid bracket {}", j),
                    }
                }
            }
        }
    }
    ret
}

fn part2(input: Vec<&str>) -> u128 {
    let pair = HashMap::from([('<','>'),('{','}'),('[',']'),('(',')')]);
    let mut result: Vec<u128> = Vec::with_capacity(input.len());
    'next_iter: for i in input {
        let mut brk: Vec<char> = Vec::new();
        for j in i.chars() {
            if pair.contains_key(&j) {
                brk.push(j);
            } else {
                let last = brk.pop().unwrap();
                if pair[&last] != j {
                    continue 'next_iter;
                }
            }
        }
        let mut ret = 0;
        while !brk.is_empty() {
            let last = brk.pop().unwrap();
            ret *= 5;
            ret += match last {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Invalid bracket {}", last),
            }
        }
        result.push(ret);
    }
    result.sort();
    result[result.len() / 2]
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();
    println!("Part1: {}\nPart2: {}", part1(input.clone()), part2(input.clone()));
}
