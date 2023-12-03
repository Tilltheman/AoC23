use std::{env, process};

mod one;
mod two;
mod three;

fn main() {
    let day: u32;

    let args = env::args().collect::<Vec<String>>();

    if args.len() == 2 {
        day = match args[1].parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error parsing day number: \"{}\"", args[1]);
                process::exit(1);
            }
        }
    } else {
        println!("usage: advent_of_code_2023 <day>");
        process::exit(1);
    }

    // Run soluntion(s) for that day
    println!("Advent of Code 2023 --- Day {}", day);

    match day {
        1 => one::solve(),
        2 => two::solve(),
        3 => three::solve(),
        _ => {
            println!("Day {} not (yet) solved.", day);
        }
    }
}
