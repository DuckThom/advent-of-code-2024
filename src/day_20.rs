use pathfinding::prelude::dijkstra;
use std::collections::{HashMap, HashSet};

pub fn execute(input: &str) {
    println!("Part 1: {}", run(&input, 100, 2));
    println!("Part 2: {}", run(&input, 100, 20));
}

type Maze = Vec<Vec<bool>>;

fn run(input: &str, required_time_save: usize, skippable: usize) -> usize {
    let (start, end, mut maze) = parse_input(input);

    block_dead_ends(&mut maze, &start, &end);

    let cheats: HashMap<usize, usize> = cheat_the_maze(&mut maze, &start, &end, skippable);

    count_cheats(&cheats, &required_time_save)
}

fn count_cheats(cheats: &HashMap<usize, usize>, required_time_save: &usize) -> usize {
    cheats.into_iter().fold(0, |acc, (score, count)| {
        if score >= required_time_save {
            return acc + count;
        }

        acc
    })
}

fn cheat_the_maze(maze: &Maze, start: &Pos, end: &Pos, skippable: usize) -> HashMap<usize, usize> {
    let (true_path, score) = do_the_dijkstra_thing(&maze, &start, &end).unwrap();

    let mut cheats: HashMap<usize, usize> = HashMap::new();
    let mut visited: HashSet<Pos> = HashSet::new();

    true_path.iter().for_each(|p| {
        visited.insert(*p);

        let jumpable_targets = p.can_jump_to(&true_path, &skippable);

        for target in jumpable_targets {
            if !visited.contains(&target) {
                let start_index = true_path.iter().position(|p| p == &target).unwrap();
                let cheated_path = true_path[start_index..].to_vec();

                let cs = visited.len() + cheated_path.len();
                if cs < score {
                    cheats.entry(score - cs).or_insert(0);
                    cheats.entry(score - cs).and_modify(|count| {
                        *count += 1;
                    });
                }
            }
        }
    });

    cheats
}

fn do_the_dijkstra_thing(
    maze: &Maze,
    start: &Pos,
    end: &Pos,
) -> Option<(Vec<Pos>, usize)> {
    dijkstra(
        start,
        |p| p.successors(&maze),
        |p| p == end,
    )
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, maze: &Maze) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut next: Vec<Pos> = vec![];

        if y > 0 && maze[y - 1][x] {
            next.push(Pos(x, y - 1));
        }

        if y < maze.len() && maze[y + 1][x] {
            next.push(Pos(x, y + 1));
        }

        if x > 0 && maze[y][x - 1] {
            next.push(Pos(x - 1, y));
        }

        if x < maze[y].len() - 1 && maze[y][x + 1] {
            next.push(Pos(x + 1, y));
        }

        next.into_iter().map(|p| (p, 1)).collect()
    }

    fn can_jump_to(&self, path: &Vec<Pos>, skippable: &usize) -> HashSet<Pos> {
        let &Pos(x, y) = self;

        let mut jumpable_positions: HashSet<Pos> = HashSet::new();

        for dy in 0..=*skippable {
            let dx = *skippable - dy;

            let candidates = vec![
                (x.wrapping_sub(dx), y.wrapping_sub(dy)), // Top-left
                (x + dx, y.wrapping_sub(dy)),             // Top-right
                (x.wrapping_sub(dx), y + dy),             // Bottom-left
                (x + dx, y + dy),                         // Bottom-right
            ];

            for (nx, ny) in candidates {
                let new_pos = Pos(nx, ny);

                if path.contains(&new_pos) {
                    jumpable_positions.insert(new_pos);
                }
            }
        }

        jumpable_positions
    }
}

use colored::Colorize;

#[allow(dead_code)]
fn print_maze(maze: &Maze, start: &Pos, end: &Pos, path: &Vec<Pos>, jumpable_positions: &Vec<Pos>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let char_to_print: &str;

            if *start == Pos(x, y) {
                char_to_print = "S";
            } else if *end == Pos(x, y) {
                char_to_print = "E";
            } else {
                if path.iter().find(|p| **p == Pos(x, y)).is_some() {
                    char_to_print = "*";
                } else {
                    char_to_print = if *cell { "." } else { "█" };
                }
            }

            print!("{}", if jumpable_positions.contains(&Pos(x, y)) {
                Colorize::red("J")
            } else {
                Colorize::white(char_to_print)
            });
        }
        print!("\n");
    }
}

fn block_dead_ends(maze: &mut Maze, start: &Pos, end: &Pos) {
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

fn count_paths(maze: &Maze, (x, y): (usize, usize)) -> usize {
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

fn parse_input(input: &str) -> (Pos, Pos, Maze) {
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

    const TEST_INPUT: &str = include_str!("../inputs/day_20/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(run(TEST_INPUT, 10, 2), 10);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(run(TEST_INPUT, 50, 20), 45);
    }
}
