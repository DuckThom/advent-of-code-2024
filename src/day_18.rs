pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input, 71, 71, 1024));
    println!("Part 2: {}", part2(&input, 71, 71).unwrap());
}

use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::collections::HashSet;

fn part1(input: &str, rows: usize, cols: usize, steps: usize) -> usize {
    let bytes = parse_input(input);
    let chunk = &bytes[0..steps];

    let start: (usize, usize) = (0, 0);
    let (path, _) = do_the_dijkstra(chunk, start, cols, rows).unwrap();

    #[cfg(test)]
    print_grid(chunk, rows, cols, &path);

    path.len() - 1
}

fn part2(input: &str, rows: usize, cols: usize) -> Option<String> {
    let bytes = parse_input(input);
    let start: (usize, usize) = (0, 0);
    let mut possible_path_tiles: HashSet<(usize, usize)> = HashSet::new();

    for steps in 1..bytes.len() {
        let chunk = &bytes[0..steps];
        let last = chunk.last();

        // If the byte didn't fall on a known path, it won't cause a blockage (yet)
        if last.is_some()
            && !possible_path_tiles.is_empty()
            && !possible_path_tiles.contains(last.unwrap())
        {
            continue;
        }

        if let Some((path, _)) = do_the_dijkstra(chunk, start, cols, rows) {
            path.iter().for_each(|p| {
                possible_path_tiles.insert(*p);
            });
        } else {
            let (x, y) = last.unwrap();

            return Some(format!("{},{}", x, y));
        }
    }

    None
}

fn do_the_dijkstra(
    chunk: &[(usize, usize)],
    start: (usize, usize),
    cols: usize,
    rows: usize,
) -> Option<(Vec<(usize, usize)>, usize)> {
    dijkstra(
        &start,
        |p| -> Vec<((usize, usize), usize)> {
            let mut succ: Vec<(usize, usize)> = vec![];
            if p.0 > 0 {
                succ.push((p.0 - 1, p.1));
            }
            if p.0 < cols - 1 {
                succ.push((p.0 + 1, p.1));
            }
            if p.1 > 0 {
                succ.push((p.0, p.1 - 1));
            }
            if p.1 < rows - 1 {
                succ.push((p.0, p.1 + 1));
            }

            succ.into_iter()
                .filter(|p| !chunk.contains(p))
                .map(|p| (p, 1))
                .collect()
        },
        |p| *p == (cols - 1, rows - 1),
    )
}

#[allow(dead_code)]
fn print_grid(bytes: &[(usize, usize)], rows: usize, cols: usize, path: &Vec<(usize, usize)>) {
    for y in 0..rows {
        for x in 0..cols {
            if path.contains(&(x, y)) {
                print!("O");
            } else if bytes.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let matcher: Regex = Regex::new("^(\\d+),(\\d+)$").unwrap();

    let mut bytes: Vec<(usize, usize)> = vec![];

    input.lines().for_each(|line| {
        let matches = matcher.captures(line).unwrap();

        let pos_x: usize = matches.get(1).unwrap().as_str().parse().unwrap();
        let pos_y: usize = matches.get(2).unwrap().as_str().parse().unwrap();

        bytes.push((pos_x, pos_y));
    });

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_18/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 7, 7, 12), 22);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 7, 7).unwrap(), "6,1");
    }
}
