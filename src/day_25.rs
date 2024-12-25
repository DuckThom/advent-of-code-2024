pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (keys, locks) = parse_input(input);
    let mut keys_that_fit: usize = 0;

    locks.iter().for_each(|lock| {
        keys.iter().for_each(|key| {
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    return;
                }
            }

            keys_that_fit += 1;
        })
    });

    keys_that_fit
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
enum InputType {
    None,
    Key,
    Lock
}

fn parse_input(input: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut keys: Vec<[usize; 5]> = Vec::new();
    let mut locks: Vec<[usize; 5]> = Vec::new();

    let mut input_type: InputType = InputType::None;
    let mut buffer: [usize; 5] = [0; 5];
    let mut line_count: usize = 0;

    input.lines().for_each(|l| {
        if l == "....." && input_type == InputType::None {
            input_type = InputType::Key;
            return;
        }

        if l == "#####" && input_type == InputType::None {
            input_type = InputType::Lock;
            return;
        }

        if l.is_empty() {
            if input_type == InputType::Key {
                keys.push(buffer);
            } else if input_type == InputType::Lock {
                locks.push(buffer);
            }
            line_count = 0;
            input_type = InputType::None;
            buffer = [0; 5];

            return;
        }

        if line_count < 5 {
            l.chars().enumerate().for_each(|(i, c)| {
                if c == '#' {
                    buffer[i] += 1;
                }
            });
            line_count += 1;
        }
    });

    (keys, locks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_25/test");
    // const P2_TEST_INPUT: &str = include_str!("../inputs/day_22/p2_test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(P2_TEST_INPUT), 23);
    // }
}
