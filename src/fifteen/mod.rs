use std::str::FromStr;

fn calculate_hash(step: &str) -> u32 {
    let mut current_val = 0;
    for c in step.chars() {
        current_val += c as u32;
        current_val *= 17;
        current_val = current_val % 256;
    }
    current_val
}

#[derive(PartialEq,Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl FromStr for Lens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(());
        }
        let label = s[0..2].to_string();
        let focal_length= s.chars().nth(3).unwrap().to_string().parse::<u32>().unwrap();
        Ok(Lens { label: label, focal_length: focal_length })
    }

}

fn solve1(input: &str) -> u32 {
    let sequence: Vec<&str> = input.strip_suffix("\n").unwrap().split(",").collect();
    let mut sum = 0;
    for element in sequence {
        sum += calculate_hash(element);
    }
    sum
}

fn solve2(input: &str) -> u128 {
    let sequence: Vec<&str> = input.strip_suffix("\n").unwrap().split(",").collect();
    let mut sum 0;
    let mut boxes: Vec<Vec<Lens>> = vec![vec![];256];
    for element in sequence {
        if element.contains("=") {
            let lens = Lens::from_str(element);
            let hash = calculate_hash(lens.label);
            boxes[hash].push(lens);
        } else {
            // Remove
        }

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
    use crate::fifteen::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

    #[test]
    fn sample_solved1() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 1320);
    }

    #[test]
    fn sample_solved2() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 30);
    }

    #[test]
    fn test_hash_calculation() {
        let res = calculate_hash("HASH");
        assert_eq!(res, 52);
    }

    #[test]
    fn test_hash_for_first_sample() {
        let res = calculate_hash("rn");
        assert_eq!(res,0)
    }

    #[test]
    fn test_lens_creation_from_str() {
        let res = Lens::from_str("cm=1").unwrap();
        assert_eq!(res, Lens { label: "cm".to_string(), focal_length: 1});
    }
}
