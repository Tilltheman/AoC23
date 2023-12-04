use regex::Regex;

fn solve1(input: &str) -> u32 {
    let lines = input.lines();
    let re_line = Regex::new(r"Card\s+(?<id>\d+):(?<winning>[0-9\s]+)\|(?<mine>[0-9\s]+)").unwrap();
    let mut sum = 0;
    for line in lines {
        let caps = re_line.captures(line).unwrap();
        let re_nums = Regex::new(r"\d+").unwrap();
        let mut winning_cards: Vec<u32> = vec![];
        let mut my_cards: Vec<u32> = vec![];
        for val in re_nums.find_iter(&caps["winning"]) {
            winning_cards.push(val.as_str().parse::<u32>().unwrap());
        };
        for val in re_nums.find_iter(&caps["mine"]) {
            my_cards.push(val.as_str().parse::<u32>().unwrap());
        };
        let mut card_sum = 0;
        let mut first = true;
        for val in winning_cards {
            if my_cards.contains(&val) {
                if first {
                    card_sum = 1;
                    first = false
                } else {
                    card_sum *= 2;
                };
            };
        };
        sum += card_sum;
    };
    sum
}

fn solve2(input: &str) -> u128 {
    let lines = input.lines();
    let re_line = Regex::new(r"Card\s+(?<id>\d+):(?<winning>[0-9\s]+)\|(?<mine>[0-9\s]+)").unwrap();
    let mut sum = 0;
    let initial_amount_cards = lines.clone().count();
    let mut amount_cards: Vec<u128> = vec![1; initial_amount_cards];
    // Original cardset
    for (i, line) in lines.enumerate() {
        let caps = re_line.captures(line).unwrap();
        let re_nums = Regex::new(r"\d+").unwrap();
        let mut winning_cards: Vec<u128> = vec![];
        let mut my_cards: Vec<u128> = vec![];
        for val in re_nums.find_iter(&caps["winning"]) {
            winning_cards.push(val.as_str().parse::<u128>().unwrap());
        };
        for val in re_nums.find_iter(&caps["mine"]) {
            my_cards.push(val.as_str().parse::<u128>().unwrap());
        };
        let mut card_wins = 0;
        for val in winning_cards {
            if my_cards.contains(&val) {
                    card_wins += 1;
            };
        };
        for j in 0..card_wins {
            amount_cards[i+1+j] += amount_cards[i];
        };
    };
    for cards in amount_cards {
        sum += cards;
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
    use crate::four::solve1;
    use crate::four::solve2;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn sample_solved1() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 13);
    }

    #[test]
    fn sample_solved2() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 30);
    }
}
