use regex::Regex;

fn is_config_possible(line: &str, part2: bool) -> (bool, u32) {
    let re_input = Regex::new(r"^Game (?<id>\d+): (?<subsets>.*)").unwrap();
    let Some(caps) = re_input.captures(line) else {
        println!("Didn't find Game id:");
        return (false, 0);
    };
    let id = caps["id"].parse().unwrap();
    // println!("ID: {}, subset: {}", &caps["id"], &caps["subsets"]);
    let re_subset = Regex::new(r"(((?<amount>\d+) (?<color>[a-z]+))[,;]*)+").unwrap();
    let mut results = vec![];
    for (_, [_, _, amount, color]) in re_subset.captures_iter(line).map(|c| c.extract()) {
        results.push((amount.parse::<u32>().unwrap(), color));
    }
    if !part2 {
        for res in results {
            // println!("Res: {} {}", res.0, res.1);
            match res.1 {
                "blue" => if res.0 > 14 { return (false, id) },
                "red" => if res.0 > 12 { return (false, id) },
                "green" => if res.0 > 13 { return (false, id) },
                _ => todo!(),
            }
        }
        (true, id)
    } else {
        let (mut b_max, mut r_max, mut g_max) = (0, 0, 0);
        for res in results {
            // println!("Res: {} {} ", res.0, res.1);
            match res.1 {
                "blue" => if res.0 > b_max { b_max = res.0 },
                "red" => if res.0 > r_max { r_max = res.0 },
                "green" => if res.0 > g_max { g_max = res.0 },
                _ => todo!(),
            }
        }
        let power = b_max * r_max * g_max;
        (true, power)
    }
}

fn solve_part_1(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let (possible, id) = is_config_possible(line, false);
        if possible {
            sum += id;
        }
    };
    sum
}

fn solve_part_2(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let (possible, power) = is_config_possible(line, true);
        if possible {
            sum += power;
        }
    };
    sum
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution part 1: {}", solve_part_1(input));
    println!("Solution part 2: {}", solve_part_2(input));
}

#[cfg(test)]

mod tests {
    use crate::two::is_config_possible;

    #[test]
    fn id_correct() {
        let line = "Game 2: 1 blue";
        let (_, res) = is_config_possible(line, false);
        assert_eq!(res, 2);
    }

    #[test]
    fn subset_possible() {
        let line = "Game 2: 1 blue";
        let (res, _) = is_config_possible(line, false);
        assert_eq!(res, true);
    }
}
