use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (start, end, mut maze) = parse_input(input);

    block_dead_ends(&mut maze, &start, &end);

    let (path, score) = dijkstra(
        &Path(start.clone(), Direction::Right),
        |p| p.successors(&maze, None),
        |p| p.0 == end,
    )
    .unwrap();

    print_maze(&maze, &start, &end, &path);

    score
}

fn do_the_dijkstra_thing(
    maze: &Vec<Vec<bool>>,
    start: &Path,
    end: &Pos,
    remove: Option<Pos>,
) -> Option<(Vec<Path>, usize)> {
    dijkstra(start, |p| p.successors(&maze, remove), |p| p.0 == *end)
}

fn part2(input: &str) -> usize {
    let (start, end, mut maze) = parse_input(input);

    block_dead_ends(&mut maze, &start, &end);

    let (path, _score) =
        do_the_dijkstra_thing(&maze, &Path(start.clone(), Direction::Right), &end, None).unwrap();

    let mut visited_tiles: HashSet<Pos> = path.iter().map(|p| p.0).collect();

    for i in 1..path.len() - 1 {
        let position = path[i];
        let next_position = path[i + 1];
        let next_paths: Vec<Path> = position
            .successors(&maze, None)
            .iter()
            .fold(vec![], |mut acc, (s, _)| {
                acc.push(*s);

                acc
            });

        if next_paths.len() > 0 {
            let (_, expected_score) = do_the_dijkstra_thing(&maze, &position, &end, None).unwrap();

            for _ in next_paths {
                let mut adjusted_maze = maze.clone();
                adjusted_maze[next_position.0 .1][next_position.0 .0] = false;

                if let Some((next_path_path, next_path_score)) =
                    do_the_dijkstra_thing(&adjusted_maze, &position, &end, Some(next_position.0))
                {
                    if next_path_score <= expected_score {
                        next_path_path.iter().for_each(|npp| {
                            visited_tiles.insert(npp.0);
                        });
                    }
                }
            }
        }
    }

    print_maze_all(&maze, &start, &end, &visited_tiles);

    // Return the count of unique tiles
    visited_tiles.len()
}

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
    fn successors(&self, maze: &Vec<Vec<bool>>, remove: Option<Pos>) -> Vec<(Path, usize)> {
        let &Path(Pos(x, y), dir) = self;
        let mut next: Vec<Path> = vec![];

        if y > 0 && maze[y - 1][x] && dir != Direction::Down {
            next.push(Path(Pos(x, y - 1), Direction::Up));
        }

        if maze[y + 1][x] && dir != Direction::Up {
            next.push(Path(Pos(x, y + 1), Direction::Down));
        }

        if x > 0 && maze[y][x - 1] && dir != Direction::Right {
            next.push(Path(Pos(x - 1, y), Direction::Left));
        }

        if x < maze[y].len() - 1 && maze[y][x + 1] && dir != Direction::Left {
            next.push(Path(Pos(x + 1, y), Direction::Right));
        }

        next.into_iter()
            .filter(|p| {
                if let Some(rem_pos) = remove {
                    p.0 != rem_pos
                } else {
                    true
                }
            })
            .map(|p| (p, if dir != p.1 { 1001 } else { 1 }))
            .collect()
    }
}

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
                    print!("{}", if *cell { '.' } else { '█' });
                }
            }
        }
        print!("\n");
    }
}

use colored::Colorize;

fn print_maze_all(maze: &Vec<Vec<bool>>, start: &Pos, end: &Pos, path: &HashSet<Pos>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *start == Pos(x, y) {
                print!("S");
            } else if *end == Pos(x, y) {
                print!("E");
            } else if !*cell {
                print!("{}", Colorize::bright_black("█"));
            } else if path.contains(&Pos(x, y)) {
                print!("{}", Colorize::bright_green("O"));
            } else {
                print!(
                    "{}",
                    if *cell {
                        Colorize::bright_black(" ")
                    } else {
                        Colorize::bright_black("█")
                    }
                );
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

    const SMALL_TEST_INPUT: &str = include_str!("../inputs/day_16/small_test");
    const LARGE_TEST_INPUT: &str = include_str!("../inputs/day_16/large_test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(SMALL_TEST_INPUT), 7036);
        assert_eq!(part1(LARGE_TEST_INPUT), 11048);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(SMALL_TEST_INPUT), 45);
        assert_eq!(part2(LARGE_TEST_INPUT), 64);
    }
}
