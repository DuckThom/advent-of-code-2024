use crate::utils;
use std::collections::HashMap;

pub fn execute() {
    let input: String = utils::read_input_file(11);

    utils::print_day_banner(11);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut number_map = parse_input(input);

    do_the_number_loop(&mut number_map, 25)
}

fn part2(input: &str) -> usize {
    let mut number_map = parse_input(input);

    do_the_number_loop(&mut number_map, 75)
}

fn do_the_number_loop(map: &mut HashMap<usize, usize>, blinks: usize) -> usize {
    for _ in 0..blinks {
        let blink_map: HashMap<usize, usize> = map.clone();

        blink_map.iter().for_each(|(key, count)| {
            let item_as_string = key.to_string();

            if *key == 0 {
                map.insert(1, *count + map.get(&1).unwrap_or(&0));
            } else if *key == 1 {
                map.insert(2024, *count + map.get(&2024).unwrap_or(&0));
            } else if item_as_string.len() % 2 == 0 {
                let (left, right) = item_as_string.split_at(item_as_string.len() / 2);

                let new_key1 = left.parse::<usize>().unwrap();
                let new_key2 = right.parse::<usize>().unwrap();

                if new_key1 == new_key2 {
                    map.insert(new_key1, (*count * 2) + map.get(&new_key1).unwrap_or(&0));
                } else {
                    map.insert(new_key1, *count + map.get(&new_key1).unwrap_or(&0));
                    map.insert(new_key2, *count + map.get(&new_key2).unwrap_or(&0));
                }
            } else {
                let new_key = key * 2024;

                map.insert(new_key, *count + map.get(&new_key).unwrap_or(&0));
            }

            map.insert(*key, map.get(&*key).unwrap_or(&0) - *count);
        });
    }

    map.iter().fold(0, |acc, (_, v)| acc + *v)
}

fn parse_input(input: &str) -> HashMap<usize, usize> {
    input.split_whitespace().fold(HashMap::new(), |mut acc, l| {
        acc.insert(l.parse::<usize>().unwrap(), 1);

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_11/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 55312);
    }
}
