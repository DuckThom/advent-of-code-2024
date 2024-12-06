use crate::utils;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn execute() {
    let input: String = utils::read_input_file(6);

    utils::print_day_banner(6);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_position(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up if y > 0 => Some((x, y - 1)),
            Direction::Down => Some((x, y + 1)),
            Direction::Left if x > 0 => Some((x - 1, y)),
            Direction::Right => Some((x + 1, y)),
            _ => None,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn part1(input: &str) -> usize {
    let matrix = parse_input(input);

    get_unique_guard_locations(&matrix).len()
}

fn part2(input: &str) -> i32 {
    let matrix = parse_input(input);

    let start_position: (usize, usize) = find_start_position(&matrix).unwrap();
    let walls: HashSet<(usize, usize)> = find_walls(&matrix);
    let path: HashSet<(usize, usize)> = get_unique_guard_locations(&matrix);

    let count = Arc::new(Mutex::new(0));

    let handles: Vec<JoinHandle<()>> = path
        .into_iter()
        .map(|location| {
            let mut new_walls: HashSet<(usize, usize)> = walls.clone();
            new_walls.insert(location);

            let thread_matrix = matrix.clone();
            let thread_count = Arc::clone(&count);

            thread::spawn(move || {
                if is_infinite_loop(start_position, &new_walls, &thread_matrix) {
                    let mut num = thread_count.lock().unwrap();
                    *num += 1;
                }
            })
        })
        .collect();

    // Wait for threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let x = *count.lock().unwrap();
    x
}

fn is_out_of_bounds((x, y): (usize, usize), matrix: &Vec<Vec<char>>) -> bool {
    y >= matrix.len() || x >= matrix[y].len()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start_position(matrix: &Vec<Vec<char>>) -> Result<(usize, usize), &str> {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == '^' {
                return Ok((x, y));
            }
        }
    }

    Err("No start position found")
}

fn find_walls(matrix: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut walls: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == '#' {
                walls.insert((x, y));
            }
        }
    }

    walls
}

fn get_unique_guard_locations(matrix: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut direction: Direction = Direction::Up;
    let mut position: (usize, usize) = find_start_position(&matrix).unwrap();
    let walls: HashSet<(usize, usize)> = find_walls(&matrix);
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    path.insert(position);

    loop {
        if let Some(next_position) = direction.next_position(position) {
            if is_out_of_bounds(next_position, &matrix) {
                break;
            }

            if !walls.contains(&next_position) {
                position = next_position;

                path.insert(position);
            } else {
                direction = direction.turn_right()
            }
        }
    }

    path
}

fn is_infinite_loop(
    start_position: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    matrix: &Vec<Vec<char>>,
) -> bool {
    let mut position = start_position.clone();
    let mut direction: Direction = Direction::Up;
    let mut path: HashSet<(usize, usize, Direction)> = HashSet::new();

    path.insert((position.0, position.1, direction));

    loop {
        if let Some(next_position) = direction.next_position(position) {
            if is_out_of_bounds(next_position, &matrix) {
                return false;
            }

            if !walls.contains(&next_position) {
                position = next_position;

                if path.contains(&(position.0, position.1, direction)) {
                    return true;
                }

                path.insert((position.0, position.1, direction));
            } else {
                direction = direction.turn_right()
            }
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_6/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}
