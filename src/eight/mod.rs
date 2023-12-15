use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;
use num::integer::lcm;

fn solve1(input: &str) -> u128 {
    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    let split_lines: Vec<&str> = input.split("\n\n").collect();
    // Pattern is the first line
    let char_vec: Vec<char> = split_lines[0].chars().collect();
    let mut pattern = VecDeque::from(char_vec);
    // println!("{pattern:?}");
    // Afterwards only nodes follow
    let node_re = Regex::new(r"(?<node_name>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
    for line in split_lines[1].lines() {
        let caps = node_re.captures(line).unwrap();
        hashmap.insert(caps.name("node_name").unwrap().as_str(),(caps.name("left").unwrap().as_str(), caps.name("right").unwrap().as_str()));
    }
    let mut sum = 0;
    let rotate_further = true;
    let mut next = hashmap.get_key_value("AAA").unwrap();
    while rotate_further {
        if pattern.front() == Some(&'L') {
            // left
            let l_next = next.1.0;
            next = hashmap.get_key_value(l_next).unwrap();
        } else if pattern.front() == Some(&'R') {
            // right
            let r_next = next.1.1;
            next = hashmap.get_key_value(r_next).unwrap();
        } else {
            panic!();
        }
        sum += 1;
        // println!("{:?}", next);
        if next.0 == &"ZZZ" {
            break;
        }
        pattern.rotate_left(1);
    }
    //println!("{hashmap:?}");
    sum
}

#[allow(dead_code)]
fn parse_input(input: &str) -> (VecDeque<char>, HashMap<&str, (&str, &str)>) {
    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    let split_lines: Vec<&str> = input.split("\n\n").collect();
    // Pattern is the first line
    let char_vec: Vec<char> = split_lines[0].chars().collect();
    let pattern = VecDeque::from(char_vec);
    let node_re = Regex::new(r"(?<node_name>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
    for line in split_lines[1].lines() {
        let caps = node_re.captures(line).unwrap();
        hashmap.insert(caps.name("node_name").unwrap().as_str(),(caps.name("left").unwrap().as_str(), caps.name("right").unwrap().as_str()));
    }
    (pattern, hashmap)
}

fn get_loop_size(mut pattern: VecDeque<char>, hashmap: HashMap<&str, (&str, &str)>, start_node: &str) -> u128 {
    let mut sum = 1;
    let mut next = hashmap.get_key_value(start_node).unwrap();
    if pattern.front() == Some(&'L') {
        next = hashmap.get_key_value(next.1.0).unwrap();
    } else {
        next = hashmap.get_key_value(next.1.1).unwrap();
    }
    pattern.rotate_left(1);
    while next.0 != &start_node {
        if pattern.front() == Some(&'L') {
            next = hashmap.get_key_value(next.1.0).unwrap();
        } else {
            next = hashmap.get_key_value(next.1.1).unwrap();
        }
        sum += 1;
        pattern.rotate_left(1);
    }
    sum
}

fn solve2(input: &str) -> u128 {
    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    let split_lines: Vec<&str> = input.split("\n\n").collect();
    // Pattern is the first line
    let char_vec: Vec<char> = split_lines[0].chars().collect();
    let pattern = VecDeque::from(char_vec);
    // println!("{pattern:?}");
    // Afterwards only nodes follow
    let node_re = Regex::new(r"(?<node_name>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
    let mut starting_points = vec![];
    let mut end_points = vec![];
    for line in split_lines[1].lines() {
        let caps = node_re.captures(line).unwrap();
        hashmap.insert(caps.name("node_name").unwrap().as_str(),(caps.name("left").unwrap().as_str(), caps.name("right").unwrap().as_str()));
        if caps.name("node_name").unwrap().as_str().ends_with("A") {
            starting_points.push(caps.name("node_name").unwrap().as_str());
        }
        if caps.name("node_name").unwrap().as_str().ends_with("Z") {
            end_points.push(caps.name("node_name").unwrap().as_str());
        }
    }
    let mut loop_sizes = vec![];
    for end in end_points {
        let loop_size = get_loop_size(pattern.clone(), hashmap.clone(), end);
        loop_sizes.push(loop_size);
    }
    let mut sum = loop_sizes[0];
    for elem in &loop_sizes[1..] {
        sum = lcm(sum, *elem);
    }
    sum
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use crate::eight::*;

    const SAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";


    const SAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_one_solved() {
        assert_eq!(2, solve1(SAMPLE));
    }

    #[test]
    fn test_one_b_solved() {
        assert_eq!(6, solve1(SAMPLE2));
    }

    #[test]
    fn test_two_solved() {
        assert_eq!(6, solve2(SAMPLE3));
    }

    #[test]
    fn test_loop_size() {
        let (pattern, hashmap) = parse_input(SAMPLE3);
        let loop_size = get_loop_size(pattern, hashmap, "11Z");
        assert_eq!(2, loop_size);
    }
    #[test]
    fn test_loop_size_2() {
        let (pattern, hashmap) = parse_input(SAMPLE3);
        let loop_size = get_loop_size(pattern, hashmap, "22Z");
        assert_eq!(3, loop_size);
    }
}
