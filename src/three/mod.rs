use regex::Regex;

fn solve1(input: &str) -> u32 {
    let line_count = input.lines().count();
    let mut sum = 0;
    let re_nums = Regex::new(r"\d+").unwrap();
    let re_non_nums = Regex::new(r"[^\d\.]").unwrap();
    let input_copy: Vec<_>= input.lines().collect();
    for (i, line) in input.lines().enumerate() {
        let iter = re_nums.find_iter(line);
        for val in iter {
            let mut num_counts = false;
            let start = val.start();
            let end = val.end();
            let n_start;
            let n_end;
            n_start = start;
            if end < 140 {
                n_end = end + 1 ;
            } else {
                n_end = end;
            };
            // check line before for non-number, non-dot chars
            if i > 0 {
                let matches = re_non_nums.find_iter(input_copy[i-1]);
                for m in matches {
                    if m.end() >= n_start && m.end() <= n_end {
                        num_counts = true;
                    };
                };
            }
            // check left and right of match
            let matches = re_non_nums.find_iter(input_copy[i]);
            for m in matches {
                if m.end() == n_start || m.end() == n_end {
                    num_counts = true;
                };
            };
            // check line after
            if i < line_count -1 {
                let matches = re_non_nums.find_iter(input_copy[i+1]);
                for m in matches {
                    if m.end() >= n_start && m.end() <= n_end {
                        num_counts = true;
                    };
                };
            }
            if num_counts {
                sum += val.as_str().parse::<u32>().unwrap();
            };
        };
    };
    sum
}

fn solve2(input: &str) -> u32 {
    let line_count = input.lines().count();
    let mut sum = 0;
    let re_star = Regex::new(r"\*").unwrap();
    let re_nums = Regex::new(r"\d+").unwrap();
    let input_copy: Vec<_>= input.lines().collect();
    for (i, line) in input.lines().enumerate() {
        let iter = re_star.find_iter(line);
        for val in iter {
            let mut adj = 0;
            let mut adj_nums = vec![];
            // line before
            if i > 0 {
                let matches = re_nums.find_iter(input_copy[i-1]);
                for m in matches {
                    if m.start() <= val.end() && val.start() <= m.end() {
                        adj += 1;
                        adj_nums.push(m.as_str().parse::<u32>().unwrap())
                    };
                };
            }
            // same line
            let matches = re_nums.find_iter(input_copy[i]);
            for m in matches {
                if m.start() <= val.end() && val.start() <= m.end() {
                    adj += 1;
                    adj_nums.push(m.as_str().parse::<u32>().unwrap())
                }
            };

            // line after
            if i < line_count - 1 {
                let matches = re_nums.find_iter(input_copy[i+1]);
                for m in matches {
                    if m.start() <= val.end() && val.start() <= m.end() {
                        adj += 1;
                        adj_nums.push(m.as_str().parse::<u32>().unwrap())
                    };
                };
            }
            if adj == 2 {
                let gear_ratio = adj_nums[0] * adj_nums[1];
                sum += gear_ratio;
            }
        };
    };
    sum
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]

mod tests {
    use crate::three::solve1;
    use crate::three::solve2;

    const SAMPLE: &str =   "467..114..\n\
                            ...*......\n\
                            ..35..633.\n\
                            ......#...\n\
                            617*......\n\
                            .....+.58.\n\
                            ..592.....\n\
                            ......755.\n\
                            ...$.*....\n\
                            .664.598..\n";

    const TESTSAMPLE1: &str = ".100...\n\
                               ..*....\n\
                               .100...\n";

    const TESTSAMPLE2: &str = "...100.\n\
                               ..*....\n\
                               100....\n";

    #[test]
    fn sample_solved1() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 4361);
    }
    #[test]
    fn sample_solved2() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 467835);
    }
    #[test]
    fn test_sample1() {
        let res = solve2(TESTSAMPLE1);
        assert_eq!(res, 10000);
    }
    #[test]
    fn test_sample2() {
        let res = solve2(TESTSAMPLE2);
        assert_eq!(res, 10000);
    }
}
