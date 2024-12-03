use crate::utils;

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

mod day_3 {
    use regex::Regex;

    pub fn part1(input: &str) -> i32 {
        let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
        let re = Regex::new(pattern).unwrap();
        let mut result: i32 = 0;

        for cap in re.captures_iter(input) {
            result += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        }

        result
    }

    pub fn part2(input: &str) -> i32 {
        let pattern = r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)";
        let re = Regex::new(pattern).unwrap();
        let mut result: i32 = 0;
        let mut multiply = true;

        for cap in re.captures_iter(input) {
            if cap[0].to_string() == "do()" {
                multiply = true;
            } else if cap[0].to_string() == "don't()" {
                multiply = false;
            } else if multiply && cap[0].starts_with("mul") {
                result += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
            }
        }

        result
    }
}
