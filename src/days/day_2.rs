pub fn execute() {
    let input = include_str!("../inputs/day_2/input");

    println!("Day 2 Part 1: {}", run_part1(input));
    println!("Day 2 Part 2: {}", run_part2(input));
}

pub fn validate() {
    let test = include_str!("../inputs/day_2/test");

    assert_eq!(run_part1(test), 2);
    assert_eq!(run_part2(test), 4);
}

fn run_part1(input: &str) -> i32 {
    let mut result: i32 = 0;

    for line in input.lines() {
        let items = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&items) {
            result += 1;
        }
    }

    result
}

fn run_part2(input: &str) -> i32 {
    let mut result: i32 = 0;

    for line in input.lines() {
        let items = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if try_is_safe(&items) {
            result += 1;
        }
    }

    result
}

fn is_safe(items: &Vec<i32>) -> bool {
    if !(1..=3).contains(&(items[1] - items[0]).abs()) {
        return false;
    }

    let increasing: bool = items[1] > items[0];

    for i in 2..items.len() {
        let is_increasing = items[i] > items[i - 1];
        if is_increasing != increasing {
            return false
        }

        if !(1..=3).contains(&(items[i] - items[i - 1]).abs()) {
            return false;
        }
    }

    true
}

fn try_is_safe(items: &Vec<i32>) -> bool {
    if is_safe(items) {
        return true;
    }

    for i in 0..items.len() {
        let mut copied = items.clone();
        copied.remove(i);

        if is_safe(&copied) {
            return true;
        }
    }

    false
}
