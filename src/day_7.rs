use crate::utils;

use std::str::FromStr;

pub fn execute() {
    let input: String = utils::read_input_file(7);

    utils::print_day_banner(7);

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let items = parse_input(input);

    items
        .iter()
        .filter(|(k, v)| check_combination(*k, 0, v))
        .map(|item| item.0)
        .sum()
}

// fn part2(input: &str) -> i32 {
//     //
// }

// Build a list of numbers like (190, [10, 19])
fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let parts = right
                .split(" ")
                .map(|i| i64::from_str(i).unwrap())
                .collect();

            (i64::from_str(left).unwrap(), parts)
        })
        .collect()
}

fn check_combination(target: i64, current: i64, numbers: &[i64]) -> bool {
    if numbers.is_empty() {
        return current == target;
    }

    if current >= target {
        return false;
    }

    let first_value = *numbers.first().unwrap();

    check_combination(target, current + first_value, &numbers[1..])
        || check_combination(target, current * first_value, &numbers[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_7/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 6);
    // }
}
