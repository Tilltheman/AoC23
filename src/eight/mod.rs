fn solve1(input: &str) -> u32 {
    0
}

fn solve2() -> u32 {
    0
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2());
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

    #[test]
    fn test_one_solved() {
        assert_eq!(1, solve1(SAMPLE));
    }
}
