use crate::utils;

pub fn execute() {
    let input = include_str!("inputs/day_1/input");

    utils::print_day_banner(1);

    println!("Part 1: {}", day_1::part1(input));
    println!("Part 2: {}", day_1::part2(input));
}

pub fn validate() {
    let test = include_str!("inputs/day_1/test");

    print!("Validating day 1... ");

    assert_eq!(day_1::part1(test), 11);
    assert_eq!(day_1::part2(test), 31);

    println!("Valid!");
}

mod day_1 {
    pub fn part1(input: &str) -> i32 {
        let (left_side, right_side) = get_sorted_list(input);

        let mut result: i32 = 0;

        for i in 0..(left_side.len()) {
            result += (left_side.get(i).unwrap() - right_side.get(i).unwrap()).abs();
        }

        result
    }

    pub fn part2(input: &str) -> i32 {
        let (left_side, right_side) = get_sorted_list(input);

        let mut result: i32 = 0;

        for i in 0..left_side.len() {
            let number_to_find = left_side.get(i).unwrap();

            let records_found = right_side.iter().filter(|&x| x == number_to_find).count();

            result += number_to_find * records_found as i32;
        }

        result
    }

    fn get_sorted_list(input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut left_side: Vec<i32> = Vec::new();
        let mut right_side: Vec<i32> = Vec::new();

        for line in input.lines() {
            let (left, right) = line.split_once("   ").unwrap();

            left_side.push(left.parse::<i32>().unwrap());
            right_side.push(right.parse::<i32>().unwrap());
        }

        left_side.sort();
        right_side.sort();

        (left_side, right_side)
    }
}
