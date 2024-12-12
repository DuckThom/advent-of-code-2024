pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_line)
        .filter(|v| is_safe(v))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_line)
        .filter(|v| try_is_safe(v))
        .count()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(str::parse::<i32>)
        .map(|f| f.expect("Encountered non-integers on a line, please fix the input"))
        .collect()
}

fn is_safe(items: &Vec<i32>) -> bool {
    if !(1..=3).contains(&(items[1] - items[0]).abs()) {
        return false;
    }

    let increasing: bool = items[1] > items[0];

    for i in 2..items.len() {
        let is_increasing = items[i] > items[i - 1];
        if is_increasing != increasing {
            return false;
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_2/test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
