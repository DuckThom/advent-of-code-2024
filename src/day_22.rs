pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let nums: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    nums.iter().fold(0, |acc, n| acc + process(n))
}

fn part2(input: &str) -> isize {
    let nums: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    // when_does_the_sequence_occur(&make_change_map(&123))

    nums.iter().fold(0, |acc, n| {
        acc + when_does_the_sequence_occur(&make_change_map(n))
    })
}

fn process(num: &usize) -> usize {
    let mut secret: usize = *num;

    for _ in 0..2000 {
        secret = transform(&secret);
    }

    secret
}

fn when_does_the_sequence_occur(changes: &Vec<(usize, isize)>) -> isize {
    for i in 0..changes.len() - 4 {
        if changes[i].1 == -2
            && changes[i + 1].1 == 1
            && changes[i + 2].1 == -1
            && changes[i + 3].1 == 3
        {
            return changes[i - 1].0 as isize - changes[i].1;
        }
    }

    0
}

fn transform(num: &usize) -> usize {
    let mut new_num = *num;

    new_num = (new_num ^ (new_num << 6)) % 16777216;
    new_num = (new_num ^ (new_num >> 5)) % 16777216;
    new_num = (new_num ^ (new_num << 11)) % 16777216;

    new_num
}

fn make_change_map(num: &usize) -> Vec<(usize, isize)> {
    let mut secret: usize = *num;
    let mut map: Vec<(usize, isize)> = vec![];

    for _ in 0..2000 {
        let new_secret = transform(&secret);
        let price_diff = (new_secret % 10) as isize - (secret % 10) as isize;

        map.push((new_secret % 10, price_diff));

        secret = new_secret;
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_TEST_INPUT: &str = include_str!("../inputs/day_22/p1_test");
    const P2_TEST_INPUT: &str = include_str!("../inputs/day_22/p2_test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(P1_TEST_INPUT), 37327623);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(P2_TEST_INPUT), 23);
    }
}
