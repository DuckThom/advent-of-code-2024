pub fn execute() {
    let input = include_str!("../inputs/day_1/input");

    println!("Day 1 Part 1: {}", run_part1(input));
    println!("Day 1 Part 2: {}", run_part2(input));
}

pub fn validate() {
    let test = include_str!("../inputs/day_1/test");

    assert_eq!(run_part1(test), 11);
    assert_eq!(run_part2(test), 31);
}

fn run_part1(input: &str) -> i32 {
    let (left_side, right_side) = get_sorted_list(input);

    let mut result: i32 = 0;

    for i in 0..(left_side.len()) {
        result += (left_side.get(i).unwrap() - right_side.get(i).unwrap()).abs();
    }

    result
}

fn run_part2(input: &str) -> i32 {
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