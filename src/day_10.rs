use crate::utils;

pub fn execute() {
    let input: String = utils::read_input_file(10);

    utils::print_day_banner(10);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let matrix = utils::input_to_usize_matrix(input);
    let start_points = get_start_points(&matrix);

    do_something(&matrix, start_points, true)
}

fn part2(input: &str) -> i32 {
    let matrix = utils::input_to_usize_matrix(input);
    let start_points = get_start_points(&matrix);

    do_something(&matrix, start_points, false)
}

fn get_start_points(matrix: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    matrix
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .for_each(|(x, _)| {
                    acc.push((x, y));
                });

            acc
        })
}

fn do_something(matrix: &Vec<Vec<usize>>, spots: Vec<(usize, usize)>, unique_only: bool) -> i32 {
    let mut score = 0;

    for spot in spots {
        let mut current_position = spot.clone();
        let mut unexplored_paths: Vec<(usize, usize)> =
            next_hiking_spots(&matrix, current_position.0, current_position.1);
        let mut end_positions: Vec<(usize, usize)> = vec![];

        while !unexplored_paths.is_empty() {
            current_position = unexplored_paths.pop().unwrap();
            if matrix[current_position.1][current_position.0] == 9 {
                if unique_only {
                    if !end_positions.contains(&current_position) {
                        end_positions.push(current_position.clone());
                        score += 1;
                    }
                } else {
                    score += 1;
                }

                continue;
            }

            let next_spots = next_hiking_spots(&matrix, current_position.0, current_position.1);
            next_spots
                .iter()
                .for_each(|spot| unexplored_paths.push(spot.clone()));
        }
    }

    score
}

fn next_hiking_spots(matrix: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let current = matrix[y][x];
    let mut spots: Vec<(usize, usize)> = vec![];

    if y > 0 && matrix[y - 1][x] == current + 1 {
        spots.push((x, y - 1));
    }

    if y < matrix.len() - 1 && matrix[y + 1][x] == current + 1 {
        spots.push((x, y + 1));
    }

    if x > 0 && matrix[y][x - 1] == current + 1 {
        spots.push((x - 1, y));
    }

    if x < matrix[0].len() - 1 && matrix[y][x + 1] == current + 1 {
        spots.push((x + 1, y));
    }

    spots
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_10/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 36);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 81);
    }
}
