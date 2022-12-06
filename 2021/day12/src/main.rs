use regex::Regex;
use std::collections::{HashMap, HashSet};

fn part1(input: Vec<(String, String)>) -> u32 {
    let mut paths: HashMap<String, Vec<String>> = {
        let mut paths: HashMap<String, Vec<String>> = HashMap::new();
        for i in input {
            if !paths.contains_key(&i.0) {
                paths.insert(i.0.clone(), vec![i.1.clone()]);
            } else {
                paths.get_mut(&i.0).unwrap().push(i.1.clone());
            }
            if !paths.contains_key(&i.1) {
                paths.insert(i.1, vec![i.0]);
            } else {
                paths.get_mut(&i.1).unwrap().push(i.0);
            }
        }
        paths
    };
    paths.remove("end");
    part1_dfs("start".to_string(), HashSet::new(), &paths)
}

fn part1_dfs(
    cur: String,
    mut visited: HashSet<String>,
    paths: &HashMap<String, Vec<String>>,
) -> u32 {
    if cur == "end" {
        return 1;
    }
    let mut count = 0;
    if cur.chars().all(char::is_lowercase) {
        visited.insert(cur.clone());
    }
    for i in paths[&cur].iter() {
        if i != "start" && !visited.contains(i) {
            // println!("{} {}", cur, i);
            count += part1_dfs(i.clone(), visited.clone(), paths);
        }
    }
    visited.remove(&cur);
    count
}

fn part2(input: Vec<(String, String)>) -> u32 {
    let mut paths: HashMap<String, Vec<String>> = {
        let mut paths: HashMap<String, Vec<String>> = HashMap::new();
        for i in input {
            if !paths.contains_key(&i.0) {
                paths.insert(i.0.clone(), vec![i.1.clone()]);
            } else {
                paths.get_mut(&i.0).unwrap().push(i.1.clone());
            }
            if !paths.contains_key(&i.1) {
                paths.insert(i.1, vec![i.0]);
            } else {
                paths.get_mut(&i.1).unwrap().push(i.0);
            }
        }
        paths
    };
    paths.remove("end");
    let visited = paths.clone().into_iter().map(|(x, _)| (x, 0_u8)).collect::<HashMap<String,u8>>();
    part2_dfs("start".to_string(), visited, &paths)
}

fn part2_dfs(
    cur: String,
    mut visited: HashMap<String, u8>,
    paths: &HashMap<String, Vec<String>>,
) -> u32 {
    if cur == "end" {
        return 1;
    }
    let mut count = 0;
    if cur.chars().all(char::is_lowercase) {
        *visited.get_mut(&cur).unwrap() += 1;
    }
    for i in paths[&cur].iter() {
        if i != "start" && visited[&cur] <= 2 {
            // println!("{} {}", cur, i);
            count += part2_dfs(i.clone(), visited.clone(), paths);
        }
    }
    visited.remove(&cur);
    count
}

fn main() {
    let re = Regex::new(r"([a-zA-Z]*)-([a-zA-Z]*)").unwrap();
    let input: Vec<(String, String)> = std::fs::read_to_string("input.txt")
        .expect("Failed to read input")
        .lines()
        .map(|l| {
            let r = re.captures(l).unwrap();
            (r[1].to_string(), r[2].to_string())
        })
        .collect();
    println!(
        "Part 1: {}\nPart 2: {}",
        part1(input.clone()),
        part2(input.clone())
    );
}
