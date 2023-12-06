use regex::Regex;

fn distance_for_held_down(seconds: u128, duration: u128) -> u128 {
    if seconds == 0 || seconds == duration {
        return 0;
    } else {
        let speed = seconds; // millimeter / seconds
        let runtime = duration - seconds;
        return runtime * speed;
    }
}

fn solve1(input: &str) -> u128 {
    let mut lines = input.lines();
    let re_nums = Regex::new(r"\d+").unwrap();
    let times: Vec<u128> = re_nums
        .find_iter(lines.next().unwrap())
        .map(|x| x.as_str().parse::<u128>().unwrap())
        .collect();
    let distances: Vec<u128> = re_nums
        .find_iter(lines.next().unwrap())
        .map(|x| x.as_str().parse::<u128>().unwrap())
        .collect();
    let mut overall_records: Vec<u128> = vec![];
    for (i, time) in times.iter().enumerate() {
        let mut records_achieved = 0;
        for j in 1..*time {
            let dist = distance_for_held_down(j, times[i]);
            if dist > distances[i] {
                records_achieved += 1;
            }
        }
        overall_records.push(records_achieved)
    }
    let mut sum = 1;
    for amount_records in overall_records {
        sum *= amount_records;
    };
    sum
}

pub fn solve() {
    let input = include_str!("input");
    let input_good_kerning = include_str!("input_good_kerning");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve1(input_good_kerning));
}

#[cfg(test)]
mod tests {
    use crate::six::solve1;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";
    const SAMPLE_GOOD_KERNING: &str = "Time:      71530
Distance:  940200";

    #[test]
    fn one_solved() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 288);
    }

    #[test]
    fn two_solved() {
        let res = solve1(SAMPLE_GOOD_KERNING);
        assert_eq!(res, 71503);
    }
}
