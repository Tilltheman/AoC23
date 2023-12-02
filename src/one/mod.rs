use regex::Regex;

fn create_number_part1(line: &str) -> u32 {
    let oline: String = String::from(line);
    let character_pos_one: usize = oline.find(char::is_numeric).unwrap();
    let character_one: char = oline.chars().nth(character_pos_one).unwrap();
    let character_pos_two: usize = oline.rfind(|c: char| c.is_numeric()).unwrap();
    let character_two: char = oline.chars().nth(character_pos_two).unwrap();
    let result_str = format!("{character_one}{character_two}");
    let result_int: u32 = result_str.parse().unwrap();
    result_int
}

fn create_number_part2(line: &str) -> u32 {
    let oline: String = String::from(line);
    let num_char_pos_1: usize = oline.find(char::is_numeric).unwrap();
    let character_1: char = oline.chars().nth(num_char_pos_1).unwrap();
    let num_char_pos_2: usize = oline.rfind(char::is_numeric).unwrap();
    let character_2: char = oline.chars().nth(num_char_pos_2).unwrap();
    let re_first = Regex::new(r"(^.*?(?<first>\d|one|two|three|four|five|six|seven|eight|nine)).*((?<last>\d|one|two|three|four|five|six|seven|eight|nine).*?$)").unwrap();
    let Some(caps) = re_first.captures(line) else {
        // println!("No Matching! Must be overlapping");
        let re_second = Regex::new(r"(^.*?(?<only>\d|one|two|three|four|five|six|seven|eight|nine)).*$").unwrap();
        let Some(caps2) = re_second.captures(line) else {
            println!("Out of ideas what happened!");
            return 0;
        };
        // println!("Matched a single time: {}, happily this only happen in digits in my data", &caps2["only"]);
        let result_str = format!("{}{}", &caps2["only"], &caps2["only"]);
        return result_str.parse().unwrap();
    };
    let first = match &caps["first"] {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        _ => "0",
    };
    let last = match &caps["last"] {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        _ => "0",
    };
    // println!("Matched first: {}, last: {}", first, last);
    let result_str2 = format!("{}{}", first, last);
    return result_str2.parse().unwrap();
}

fn solve_first() -> u32 {
    let input = include_str!("input");
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let number = create_number_part1(line);
        sum += number;
    }
    sum
}

fn solve_second() -> u32 {
    let input = include_str!("input");
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let number = create_number_part2(line);
        sum += number;
    }
    sum
}

pub fn solve() {
    println!("Solution for first part: {}", solve_first());
    println!("Solution for second part: {}", solve_second());
}
