use std::collections::HashMap;

pub fn execute(input: &str) {
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

fn split_number(num: usize) -> (usize, usize) {
    if num == 0 {
        return (0, 0);
    }

    // Calculate the total number of digits
    let num_digits = num.ilog10() as usize + 1;
    let middle = num_digits / 2;
    let divisor = 10_usize.pow(middle as u32);

    (num / divisor, num % divisor)
}

fn do_the_number_loop(map: &mut HashMap<usize, usize>, blinks: usize) -> usize {
    for _ in 0..blinks {
        let blink_map: HashMap<usize, usize> = map.clone();

        blink_map.iter().for_each(|(key, count)| {
            if *key == 0 {
                map.insert(1, *count + map.get(&1).unwrap_or(&0));
            } else if *key == 1 {
                map.insert(2024, *count + map.get(&2024).unwrap_or(&0));
            } else if (key.ilog10() as usize + 1) % 2 == 0 {
                let (new_key1, new_key2) = split_number(*key);

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
