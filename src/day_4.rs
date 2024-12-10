use crate::utils;

pub fn execute() {
    let input: String = utils::read_input_file(4);

    utils::print_day_banner(4);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let matrix = utils::input_to_char_matrix(input);

    get_start_locations(&matrix, 'X')
        .iter()
        .fold(0, |acc, loc| acc + scan_xmas_location(loc, &matrix))
}

fn part2(input: &str) -> usize {
    let matrix = utils::input_to_char_matrix(input);

    get_start_locations(&matrix, 'A')
        .iter()
        .filter(|(x, y)| is_x_mas((x, y), &matrix))
        .count()
}

fn get_start_locations(matrix: &Vec<Vec<char>>, starting_char: char) -> Vec<(usize, usize)> {
    let mut start_locations: Vec<(usize, usize)> = vec![];
    let mut y: usize = 0;

    for line in matrix {
        let mut x: usize = 0;

        for c in line {
            if *c == starting_char {
                start_locations.push((x, y));
            }

            x += 1;
        }

        y += 1;
    }

    start_locations
}

fn scan_xmas_location((x, y): &(usize, usize), matrix: &Vec<Vec<char>>) -> i32 {
    let mut hits: i32 = 0;

    hits += scan_straight((&x, &y), &matrix);
    hits += scan_diagonal((&x, &y), &matrix);

    hits
}

fn scan_straight((x, y): (&usize, &usize), matrix: &Vec<Vec<char>>) -> i32 {
    let mut hits: i32 = 0;

    // Right
    if *x < matrix[0].len() - 3 {
        let buffer: Vec<char> = vec![
            matrix[*y][*x],
            matrix[*y][*x + 1],
            matrix[*y][*x + 2],
            matrix[*y][*x + 3],
        ];

        if is_xmas(&buffer) {
            hits += 1;
        }
    }

    // Left
    if *x >= 3 {
        let buffer: Vec<char> = vec![
            matrix[*y][*x],
            matrix[*y][*x - 1],
            matrix[*y][*x - 2],
            matrix[*y][*x - 3],
        ];

        if is_xmas(&buffer) {
            hits += 1;
        }
    }

    // Down
    if *y < matrix.len() - 3 {
        let buffer: Vec<char> = vec![
            matrix[*y][*x],
            matrix[*y + 1][*x],
            matrix[*y + 2][*x],
            matrix[*y + 3][*x],
        ];

        if is_xmas(&buffer) {
            hits += 1;
        }
    }

    // Up
    if *y >= 3 {
        let buffer: Vec<char> = vec![
            matrix[*y][*x],
            matrix[*y - 1][*x],
            matrix[*y - 2][*x],
            matrix[*y - 3][*x],
        ];

        if is_xmas(&buffer) {
            hits += 1;
        }
    }

    hits
}

fn scan_diagonal((x, y): (&usize, &usize), matrix: &Vec<Vec<char>>) -> i32 {
    let mut hits: i32 = 0;

    if *y >= 3 {
        // Top right
        if *x < matrix[0].len() - 3 {
            let buffer: Vec<char> = vec![
                matrix[*y][*x],
                matrix[*y - 1][*x + 1],
                matrix[*y - 2][*x + 2],
                matrix[*y - 3][*x + 3],
            ];

            if is_xmas(&buffer) {
                hits += 1;
            }
        }

        // Top left
        if *x >= 3 {
            let buffer: Vec<char> = vec![
                matrix[*y][*x],
                matrix[*y - 1][*x - 1],
                matrix[*y - 2][*x - 2],
                matrix[*y - 3][*x - 3],
            ];

            if is_xmas(&buffer) {
                hits += 1;
            }
        }
    }

    if *y < matrix.len() - 3 {
        // Bottom right
        if *x < matrix[0].len() - 3 {
            let buffer: Vec<char> = vec![
                matrix[*y][*x],
                matrix[*y + 1][*x + 1],
                matrix[*y + 2][*x + 2],
                matrix[*y + 3][*x + 3],
            ];

            if is_xmas(&buffer) {
                hits += 1;
            }
        }

        // Bottom left
        if *x >= 3 {
            let buffer: Vec<char> = vec![
                matrix[*y][*x],
                matrix[*y + 1][*x - 1],
                matrix[*y + 2][*x - 2],
                matrix[*y + 3][*x - 3],
            ];

            if is_xmas(&buffer) {
                hits += 1;
            }
        }
    }

    hits
}

fn is_x_mas((x, y): (&usize, &usize), matrix: &Vec<Vec<char>>) -> bool {
    if *x == 0 || *y == 0 || *x == matrix[0].len() - 1 || *y == matrix.len() - 1 {
        return false;
    }

    let buffer1: String = vec![
        matrix[*y - 1][*x - 1],
        matrix[*y][*x],
        matrix[*y + 1][*x + 1],
    ]
    .iter()
    .collect();

    let buffer2: String = vec![
        matrix[*y - 1][*x + 1],
        matrix[*y][*x],
        matrix[*y + 1][*x - 1],
    ]
    .iter()
    .collect();

    (buffer1 == "MAS" || buffer1 == "SAM") && (buffer2 == "MAS" || buffer2 == "SAM")
}

fn is_xmas(buffer: &Vec<char>) -> bool {
    buffer[0] == 'X' && buffer[1] == 'M' && buffer[2] == 'A' && buffer[3] == 'S'
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_4/test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 9);
    }
}
