pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

use pathfinding::prelude::{dijkstra, dijkstra_partial};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Path(Pos, Direction);

impl Path {
    fn successors(&self, maze: &Vec<Vec<bool>>) -> Vec<(Path, usize)> {
        let &Path(Pos(x, y), dir) = self;
        let mut next: Vec<Path> = vec![];

        if y > 0 && maze[y - 1][x] {
            next.push(Path(Pos(x, y - 1), Direction::Down));
        }

        if maze[y + 1][x] {
            next.push(Path(Pos(x, y + 1), Direction::Up));
        }

        if x > 0 && maze[y][x - 1] {
            next.push(Path(Pos(x - 1, y), Direction::Left));
        }

        if x < maze[y].len() - 1 && maze[y][x + 1] {
            next.push(Path(Pos(x + 1, y), Direction::Right));
        }

        next.into_iter()
            .map(|p| (p, if dir != p.1 { 1001 } else { 1 }))
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let (start, end, mut maze) = parse_input(input);

    block_dead_ends(&mut maze, &start, &end);

    let (path, score) = dijkstra(
        &Path(start.clone(), Direction::Right),
        |p| p.successors(&maze),
        |p| p.0 == end,
    )
    .unwrap();

    print_maze(&maze, &start, &end, &path);

    score
}

// fn part2(input: &str) -> usize {
//     let (start, end, mut maze) = parse_input(input);
//
//     block_dead_ends(&mut maze, &start, &end);
//
//     let wef = dijkstra_partial(
//         &Path(start.clone(), Direction::Right),
//         |p| p.successors(&maze),
//         |p| p.0 == end,
//     );
//
//     dbg!(&wef);
//
//     // print_maze(&maze, &start, &end, &path);
//
//     0
// }

fn print_maze(maze: &Vec<Vec<bool>>, start: &Pos, end: &Pos, path: &Vec<Path>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *start == Pos(x, y) {
                print!("S");
            } else if *end == Pos(x, y) {
                print!("E");
            } else {
                if path.iter().find(|p| p.0 == Pos(x, y)).is_some() {
                    print!("*");
                } else {
                    print!("{}", if *cell { '.' } else { '#' });
                }
            }
        }
        print!("\n");
    }
}

fn block_dead_ends(maze: &mut Vec<Vec<bool>>, start: &Pos, end: &Pos) {
    let mut has_changes = true;
    let rows = maze.len();

    while has_changes {
        has_changes = false;

        // Iterate over the maze explicitly using `for` loops
        for y in 0..rows {
            if y == 0 || y == rows - 1 {
                continue;
            }

            let cols = maze[y].len(); // Number of columns in the current row
            for x in 0..cols {
                let cell = &maze[y][x];

                if !*cell || *start == Pos(x, y) || *end == Pos(x, y) {
                    continue; // Skip cells that are walls, start, or end
                }

                // Count paths on the fly, directly using the updated maze
                if count_paths(maze, (x, y)) == 1 {
                    maze[y][x] = false; // Update the maze immediately
                    has_changes = true;
                }
            }
        }
    }
}

fn count_paths(maze: &Vec<Vec<bool>>, (x, y): (usize, usize)) -> usize {
    let mut paths: usize = 0;

    if maze[y - 1][x] {
        paths += 1;
    }

    if maze[y + 1][x] {
        paths += 1;
    }

    if maze[y][x - 1] {
        paths += 1;
    }

    if maze[y][x + 1] {
        paths += 1;
    }

    paths
}

fn parse_input(input: &str) -> (Pos, Pos, Vec<Vec<bool>>) {
    let mut start: Pos = Pos(0, 0);
    let mut end: Pos = Pos(0, 0);

    let maze = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Pos(x, y);
                    }

                    if c == 'E' {
                        end = Pos(x, y);
                    }

                    c != '#'
                })
                .collect()
        })
        .collect();

    (start, end, maze)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_16/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7036);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 45);
    // }
}
