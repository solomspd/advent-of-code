use std::collections::HashSet;

fn part1(input: Vec<(Vec<&str>,Vec<&str>)>) -> u32 {
    let mut ret = 0;
    for i in input {
        for j in i.1 {
            let chr_cnt = j.chars().count();
            if chr_cnt == 2 || chr_cnt == 4 || chr_cnt == 3 || chr_cnt == 7 {
                ret += 1;
            }
        }
    }
    ret
}

fn part2(input: Vec<(Vec<&str>,Vec<&str>)>) -> u32 {
    let mut ret: u32 = 0;
    for i in input {
        let mut mapping: Vec<HashSet<char>> = vec![HashSet::new(); 10];
        let mut unsure: Vec<HashSet<char>> = Vec::with_capacity(6);
        for j in i.0 {
            let cur = HashSet::from_iter(j.chars());
            match j.chars().count() {
                2 => mapping[1] = cur,
                3 => mapping[7] = cur,
                4 => mapping[4] = cur,
                7 => mapping[8] = cur,
                _ => unsure.push(cur),
            };
        }

        mapping[6] = unsure.swap_remove(unsure.iter().position(|x| x.len() == 6 && !mapping[1].is_subset(&x)).unwrap());
        mapping[9] = unsure.swap_remove(unsure.iter().position(|x| x.len() == 6 && mapping[4].is_subset(&x)).unwrap());
        mapping[0] = unsure.swap_remove(unsure.iter().position(|x| x.len() == 6).unwrap());

        mapping[3] = unsure.swap_remove(unsure.iter().position(|x| x.len() == 5 && mapping[1].is_subset(&x)).unwrap());
        mapping[2] = unsure.swap_remove(unsure.iter().position(|x| x.len() == 5 && mapping[4].difference(&x).count() == 2).unwrap());
        mapping[5] = unsure.swap_remove(0);

        for (j,jj) in i.1.iter().rev().enumerate() {
            let th: u32 = match jj.chars().count() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => {
                    let match_char = HashSet::from_iter(jj.chars()) as HashSet<char>;
                    mapping.iter().position(|x| *x == match_char).unwrap() as u32
                },
            };
            ret += th * 10_u32.pow(j as u32);
        }
    }
    ret
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let input = input.lines().map(|l| {let mut iter = l.split(" | "); (iter.next().unwrap().split_whitespace().collect::<Vec<&str>>(), iter.next().unwrap().split_whitespace().collect::<Vec<&str>>())}).collect::<Vec<_>>();
    println!("Part1: {}\nPart2: {}", part1(input.clone()), part2(input.clone()));
}
