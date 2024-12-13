pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

use regex::Regex;

#[derive(PartialEq)]
struct Button {
    x: u64,
    y: u64,
    cost: u8,
}

#[derive(PartialEq)]
struct Target {
    x: u64,
    y: u64,
}

type Arcade = (Button, Button, Target);

fn part1(input: &str) -> u64 {
    let arcades = parse_input(input, 0);

    calculate(&arcades)
}

fn part2(input: &str) -> u64 {
    let arcades = parse_input(input, 10000000000000);

    calculate(&arcades)
}

fn calculate(arcades: &Vec<Arcade>) -> u64 {
    arcades
        .iter()
        .map(|arcade| {
            let button_a = &arcade.0;
            let button_b = &arcade.1;
            let target = &arcade.2;

            // Coefficients for equations
            let ax = button_a.x as i64;
            let ay = button_a.y as i64;
            let bx = button_b.x as i64;
            let by = button_b.y as i64;
            let px = target.x as i64;
            let py = target.y as i64;

            // Calculate the denominator (common factor in both equations)
            let denominator = by * ax - bx * ay;

            if denominator == 0 {
                return 0; // No solution possible if denominator is zero
            }

            // Compute `b` (Button B presses) using the primary equation
            let b_num = py * ax - px * ay;
            if b_num % denominator != 0 {
                return 0; // `b` is not an integer, so no solution
            }

            let b = b_num / denominator;

            if b < 0 {
                return 0; // Invalid solution (presses can't be negative)
            }

            // Compute `a` (Button A presses) using the secondary equation
            let a_num = px - b * bx;
            if a_num % ax != 0 {
                return 0; // `a` is not an integer, so no solution
            }

            let a = a_num / ax;

            if a < 0 {
                return 0; // Invalid solution (presses can't be negative)
            }

            // Calculate the total cost for this solution
            let cost = (a as u64 * button_a.cost as u64) + (b as u64 * button_b.cost as u64);
            cost
        })
        .sum()
}

fn parse_input(input: &str, target_adjustment: u64) -> Vec<Arcade> {
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
            let x = captures[2].parse::<u64>().unwrap();
            let y = captures[3].parse::<u64>().unwrap();

            if button_type == "A" {
                buffer.0 = Button { x, y, cost: 3 };
            } else if button_type == "B" {
                buffer.1 = Button { x, y, cost: 1 };
            }
        }

        if let Some(captures) = prize_regex.captures(line) {
            let x = captures[1].parse::<u64>().unwrap() + target_adjustment;
            let y = captures[2].parse::<u64>().unwrap() + target_adjustment;
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
