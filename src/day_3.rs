use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;

pub fn execute() {
    let input: String = utils::read_input_file(3);

    utils::print_day_banner(3);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

lazy_static! {
    static ref REGEX_PART1: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref REGEX_PART2: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
}

fn part1(input: &str) -> i32 {
    REGEX_PART1.captures_iter(input).fold(0, |acc, cap| {
        acc + cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
    })
}

fn part2(input: &str) -> i32 {
    let mut multiply = true;

    REGEX_PART2.captures_iter(input).fold(0, |acc, cap| {
        if cap[0] == *"do()" {
            multiply = true;
        } else if cap[0] == *"don't()" {
            multiply = false;
        } else if multiply && cap[0].starts_with("mul") {
            return acc + cap[1].parse::<i32>().unwrap_or(0) * cap[2].parse::<i32>().unwrap_or(0);
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_3/test");
    const TEST_INPUT2: &str = include_str!("../inputs/day_3/test2");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 161);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT2), 48);
    }
}
