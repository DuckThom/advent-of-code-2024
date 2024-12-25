// pub fn execute(input: &str) {
//     println!("Part 1: {}", part1(&input));
//     println!("Part 2: {}", part2(&input));
// }
// use std::collections::{HashMap, HashSet};

fn part1(_input: &str) -> usize {
    // let mut computers: HashMap<String, HashSet<String>> = HashMap::new();

    // input.lines().for_each(|l| {
    //     let (pc1, pc2) = l.split_once("-").unwrap();
    //
    //     let computer1 = computers.entry(pc1.to_string()).or_insert(HashSet::new());
    //     let computer2 = computers.entry(pc2.to_string()).or_insert(HashSet::new());
    //
    //     computer1.insert(pc2.to_string());
    //     computer2.insert(pc1.to_string());
    // });

    // dbg!(&computers);

    0
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_23/test");
    // const P2_TEST_INPUT: &str = include_str!("../inputs/day_22/p2_test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(P2_TEST_INPUT), 23);
    // }
}
