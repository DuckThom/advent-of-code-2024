use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;

pub fn execute() {
    let input = include_str!("inputs/day_3/input");

    utils::print_day_banner(3);

    println!("Part 1: {}", day_3::part1(input));
    println!("Part 2: {}", day_3::part2(input));
}

pub fn validate() {
    let test = include_str!("inputs/day_3/test");
    let test2 = include_str!("inputs/day_3/test2");

    print!("Validating day 3... ");

    assert_eq!(day_3::part1(test), 161);
    assert_eq!(day_3::part2(test2), 48);

    println!("Valid!");
}

lazy_static! {
    static ref REGEX_PART1: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref REGEX_PART2: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
}

mod day_3 {
    use super::*;

    pub fn part1(input: &str) -> i32 {
        REGEX_PART1.captures_iter(input).fold(0, |acc, cap| {
            acc + cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
        })
    }

    pub fn part2(input: &str) -> i32 {
        let mut multiply = true;

        REGEX_PART2.captures_iter(input).fold(0, |acc, cap| {
            if cap[0] == *"do()" {
                multiply = true;
            } else if cap[0] == *"don't()" {
                multiply = false;
            } else if multiply && cap[0].starts_with("mul") {
                return acc
                    + cap[1].parse::<i32>().unwrap_or(0) * cap[2].parse::<i32>().unwrap_or(0);
            }

            acc
        })
    }
}
