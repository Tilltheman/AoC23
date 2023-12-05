use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct MyMap {
    vector: Vec<(u128, u128, u128)>,
}

impl FromStr for MyMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let mut vector: Vec<(u128, u128, u128)> = vec![];
        for line in &lines[1..] {
            let v: Vec<&str> = line.split(" ").collect();
            let dest_range_start = v[0].parse::<u128>().unwrap();
            let source_range_start = v[1].parse::<u128>().unwrap();
            let range_length = v[2].parse::<u128>().unwrap();
            vector.push((source_range_start, dest_range_start, range_length));
        };
        vector.sort_by_key(|k| k.0);
        Ok(MyMap { vector })
    }
}


impl MyMap {
    fn get_value(&self, val: u128) -> u128 {
        for element in &self.vector {
            if val < element.0 || val >= element.0 + element.2 {
                continue
            } else if val == element.0 {
                return element.1;
            } else {
                return element.1 + val - element.0;
            };
        };
        val
    }
}

fn solve1(input: &str) -> u128 {
    let lines: Vec<&str> = input.lines().collect();
    let re_nums = Regex::new(r"\d+").unwrap();
    let initial_seeds: Vec<_> = re_nums.find_iter(lines[0])
        .map(|x| x.as_str().parse::<u128>().unwrap())
        .collect();
    let map_lines: Vec<&str> = input.split("\n\n").collect();
    let mut vec_of_maps: Vec<MyMap> = vec![];
    for lines in &map_lines[1..] {
        vec_of_maps.push(MyMap::from_str(lines).unwrap());
    }
    let mut min = u128::MAX;
    for seed in initial_seeds {
        let mut next = vec_of_maps[0].get_value(seed);
        for vec in &vec_of_maps[1..] {
            next = vec.get_value(next);
        };
        if min > next {
            min = next;
        };
    };
    min
}

fn solve2(input: &str) -> u128 {
    let lines: Vec<&str> = input.lines().collect();
    let re_nums = Regex::new(r"\d+").unwrap();
    let initial_seeds: Vec<_> = re_nums.find_iter(lines[0])
        .map(|x| x.as_str().parse::<u128>().unwrap())
        .collect();
    let map_lines: Vec<&str> = input.split("\n\n").collect();
    let mut vec_of_maps: Vec<MyMap> = vec![];
    for lines in &map_lines[1..] {
        vec_of_maps.push(MyMap::from_str(lines).unwrap());
    }
    // seeds are now ranges, prepare the numbers accordingly
    println!("{:?}", initial_seeds);
    let mut all_seeds: Vec<u128> = vec![];
    for seeds in initial_seeds.chunks(2) {
        println!("{:?}", seeds);
        for i in 0..seeds[1] {
            all_seeds.push(seeds[0]+i);
        }
    }
    println!("Len all_seeds: {}", all_seeds.len());
    let mut min = u128::MAX;
    let ln = all_seeds.len();
    for (i, seed) in all_seeds.into_iter().enumerate() {
        // println!("Start {}",seed);
        if i % (ln/8) == 0 {
            println!("1/8 down");
        }
        let mut next = vec_of_maps[0].get_value(seed);
        for vec in &vec_of_maps[1..] {
            //println!("{}",next);
            next = vec.get_value(next);
        };
        if min > next {
            min = next;
        };
        // println!("min: {}", min);
    };
    min
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use crate::five::solve1;
    use crate::five::solve2;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn one_solved() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 35);
    }

    #[test]
    fn two_solved() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 46);
    }
}
