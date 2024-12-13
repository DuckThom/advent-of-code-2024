pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

use regex::Regex;

#[derive(PartialEq)]
struct Button {
    x: usize,
    y: usize,
    cost: u8,
}

#[derive(PartialEq)]
struct Target {
    x: usize,
    y: usize,
}

type Arcade = (Button, Button, Target);

fn part1(input: &str) -> usize {
    let arcades = parse_input(input, 0);

    calculate(&arcades)
}

fn part2(input: &str) -> usize {
    let arcades = parse_input(input, 10000000000000);

    calculate(&arcades)
}

fn calculate(arcades: &Vec<Arcade>) -> usize {
    arcades
        .iter()
        .fold(0, |acc, arcade| {
            let button_a = &arcade.0;
            let button_b = &arcade.1;
            let target = &arcade.2;

            // Iterate over possible n1 (presses for Button A)
            for n1 in 0..=target.x / button_a.x {
                // Calculate x distance remaining for Button B
                let remaining_x = target.x as isize - (n1 * button_a.x) as isize;

                // Check if Button B can cover the remaining X distance
                if remaining_x % button_b.x as isize != 0 {
                    continue; // Skip this iteration if remaining_x is not divisible
                }

                let n2_x = remaining_x / button_b.x as isize;

                // Similarly, calculate y movements and check consistency
                let y_movement_a = n1 * button_a.y;
                let remaining_y = target.y as isize - y_movement_a as isize;

                if n2_x < 0 || remaining_y % button_b.y as isize != 0 {
                    continue;
                }

                let n2_y = remaining_y / button_b.y as isize;

                // If the derived n2_x and n2_y are not consistent, skip.
                if n2_x != n2_y {
                    continue;
                }

                let n2 = n2_x; // Both must be equal per our calculation

                if n2 < 0 {
                    continue; // Skip invalid values for n2
                }

                // Calculate the cost for this (n1, n2) configuration
                let cost = (n1 as usize * button_a.cost as usize)
                    + (n2 as usize * button_b.cost as usize);

                return acc + cost;
            }

            acc
        })
}

fn parse_input(input: &str, target_adjustment: usize) -> Vec<Arcade> {
    let mut arcades: Vec<Arcade> = vec![];
    let mut buffer: Arcade = (
        Button {
            x: 0,
            y: 0,
            cost: 0,
        },
        Button {
            x: 0,
            y: 0,
            cost: 0,
        },
        Target { x: 0, y: 0 },
    );

    let button_regex = Regex::new(r"Button ([AB]): X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    for line in input.lines() {
        if line.is_empty() {
            arcades.push(buffer);
            buffer = (
                Button {
                    x: 0,
                    y: 0,
                    cost: 0,
                },
                Button {
                    x: 0,
                    y: 0,
                    cost: 0,
                },
                Target { x: 0, y: 0 },
            );
            continue;
        }

        if let Some(captures) = button_regex.captures(line) {
            let button_type = &captures[1];
            let x = captures[2].parse::<usize>().unwrap();
            let y = captures[3].parse::<usize>().unwrap();

            if button_type == "A" {
                buffer.0 = Button { x, y, cost: 3 };
            } else if button_type == "B" {
                buffer.1 = Button { x, y, cost: 1 };
            }
        }

        if let Some(captures) = prize_regex.captures(line) {
            let x = captures[1].parse::<usize>().unwrap() + target_adjustment;
            let y = captures[2].parse::<usize>().unwrap() + target_adjustment;
            buffer.2 = Target { x, y };
        }
    }

    if buffer != (
        Button { x: 0, y: 0, cost: 0 },
        Button { x: 0, y: 0, cost: 0 },
        Target { x: 0, y: 0 },
    ) {
        arcades.push(buffer);
    }

    arcades
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_13/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 480);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 1206);
    // }
}
