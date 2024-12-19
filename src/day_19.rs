use std::collections::{HashMap, HashSet};

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (towels, combinations) = parse_input(input);

    let mut valid_count: usize = 0;
    let mut cache: HashMap<String, bool> = HashMap::new();

    for combination in combinations {
        if is_valid_combination(&combination, &towels, &mut cache) {
            valid_count += 1;
        }
    }

    valid_count
}

fn part2(input: &str) -> usize {
    let (towels, combinations) = parse_input(input);
    let mut arrangements: usize = 0;
    let mut cache: HashMap<String, usize> = HashMap::new();

    for combination in combinations {
        arrangements =
            find_all_arrangements(&combination, &towels, &mut vec![], arrangements, &mut cache);
    }

    arrangements
}

fn parse_input(input: &str) -> (HashSet<&str>, Vec<String>) {
    let mut towels: HashSet<&str> = HashSet::new();
    let mut combinations: Vec<String> = vec![];
    let mut parse_mode: u8 = 0;

    input.lines().for_each(|line| {
        if line.is_empty() {
            parse_mode = 1;
        } else if parse_mode == 0 {
            line.split(", ").for_each(|s| {
                towels.insert(s);
            });
        } else {
            combinations.push(line.to_string());
        }
    });

    (towels, combinations)
}

fn find_all_arrangements(
    combination: &str,
    towel_set: &HashSet<&str>,
    current: &mut Vec<String>,
    mut counter: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if combination.is_empty() {
        return counter + 1;
    }

    if let Some(&result) = cache.get(combination) {
        return counter + result;
    }

    for i in 1..=combination.len() {
        let prefix = &combination[..i];

        if towel_set.contains(prefix) {
            current.push(prefix.to_string());

            let combination_to_check = &combination[i..];

            let original_counter = counter;

            counter =
                find_all_arrangements(combination_to_check, towel_set, current, counter, cache);

            cache.insert(combination_to_check.to_string(), counter - original_counter);

            current.pop();
        }
    }

    counter
}

fn is_valid_combination(
    combination: &str,
    towel_set: &HashSet<&str>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if let Some(&result) = cache.get(combination) {
        return result;
    }

    if combination.is_empty() {
        return true;
    }

    for i in 1..=combination.len() {
        let prefix = &combination[..i];

        if towel_set.contains(prefix) {
            let suffix = &combination[i..];

            if is_valid_combination(suffix, towel_set, cache) {
                cache.insert(combination.to_string(), true);

                return true;
            }
        }
    }

    cache.insert(combination.to_string(), false);

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_19/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 16);
    }
}
