use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

fn solve1(input: &str) -> u32 {
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

fn solve2(input: &str) -> u32 {
    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    let split_lines: Vec<&str> = input.split("\n\n").collect();
    // Pattern is the first line
    let char_vec: Vec<char> = split_lines[0].chars().collect();
    let mut pattern = VecDeque::from(char_vec);
    // println!("{pattern:?}");
    // Afterwards only nodes follow
    let node_re = Regex::new(r"(?<node_name>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
    let mut starting_points = vec![];
    for line in split_lines[1].lines() {
        let caps = node_re.captures(line).unwrap();
        hashmap.insert(caps.name("node_name").unwrap().as_str(),(caps.name("left").unwrap().as_str(), caps.name("right").unwrap().as_str()));
        if caps.name("node_name").unwrap().as_str().ends_with("A") {
            starting_points.push(caps.name("node_name").unwrap().as_str());
        }
    }
    // println!("{:?}", starting_points);
    let mut sum = 0;
    let mut rotate_further = true;
    let mut next = vec![];
    for start in starting_points {
        next.push(hashmap.get_key_value(start).unwrap().clone());
    }
    while rotate_further {
        // println!("{:?} {:?}", pattern.front(), next);
        for nnext in next.iter_mut() {
            if pattern.front() == Some(&'L') {
                // left
                *nnext = hashmap.get_key_value(nnext.1.0).unwrap();
            } else if pattern.front() == Some(&'R') {
                // right
                *nnext = hashmap.get_key_value(nnext.1.1).unwrap();
            } else {
                panic!();
            }
        }
        sum += 1;
        rotate_further = false;
        // println!("{:?}", next);
        for nnext in next.iter_mut() {
            if ! nnext.0.ends_with("Z") {
                rotate_further = true;
                break;
            }
        }
        pattern.rotate_left(1);
    }
    //println!("{hashmap:?}");
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
        assert_eq!(7, solve2(SAMPLE3));
    }
}
