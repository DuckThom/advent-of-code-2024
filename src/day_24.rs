use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (mut registers, mut actions) = parse_input(input);

    while !actions.is_empty() {
        let (source1, action, source2, target) = actions.pop_front().unwrap();

        if !registers.contains_key(&source1) || !registers.contains_key(&source2) {
            // Cannot process this action yet, some registers are not set
            actions.push_back((source1, action, source2, target));

            continue;
        }

        if action == ActionType::AND {
            registers.insert(target, registers[&source1] && registers[&source2]);
        } else if action == ActionType::OR {
            registers.insert(target, registers[&source1] || registers[&source2]);
        } else if action == ActionType::XOR {
            registers.insert(target, registers[&source1] ^ registers[&source2]);
        }
    }

    let binary_string: String = registers
        .iter()
        .filter(|(key, _value)| key.starts_with('z'))
        .map(|(key, value)| (key.clone(), *value))
        .sorted_by(|(a, _), (b, _)| b.cmp(a))
        .map(|(_key, value)| if value { '1' } else { '0' })
        .collect();

    usize::from_str_radix(&binary_string, 2).unwrap_or(0)
}

// fn part2(input: &str) -> isize {
//     let nums: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
//
//     // when_does_the_sequence_occur(&make_change_map(&123))
//
//     nums.iter().fold(0, |acc, n| {
//         acc + when_does_the_sequence_occur(&make_change_map(n))
//     })
// }

#[derive(Debug, Eq, PartialEq)]
enum ActionType {
    AND,
    OR,
    XOR,
}

type Registers = HashMap<String, bool>;
type Action = (String, ActionType, String, String);

fn parse_input(input: &str) -> (Registers, VecDeque<Action>) {
    let mut registers: Registers = HashMap::new();
    let mut actions: VecDeque<Action> = VecDeque::new();
    let mut process_type: usize = 0;

    input.lines().for_each(|l| {
        if l.is_empty() {
            process_type = 1;
            return;
        }

        if process_type == 0 {
            let (register, value) = l.split_once(": ").unwrap();

            registers.insert(register.to_string(), value == "1");
        } else if process_type == 1 {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let source1 = parts[0].to_string();
            let action = match parts[1] {
                "AND" => ActionType::AND,
                "OR" => ActionType::OR,
                "XOR" => ActionType::XOR,
                _ => panic!("Unknown action: {}", parts[1]),
            };
            let source2 = parts[2].to_string();
            let target = parts[4].to_string();

            actions.push_back((source1, action, source2, target));
        }
    });

    (registers, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_24/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2024);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(P2_TEST_INPUT), 23);
    // }
}
