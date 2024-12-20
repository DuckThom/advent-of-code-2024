use colored::Colorize;
use pathfinding::prelude::dijkstra;
use std::collections::{HashMap, HashSet};

pub fn execute(input: &str) {
    println!("Part 1: {}", run(&input, 100, 2));
    println!("Part 2: {}", run(&input, 100, 20));
}

type Maze = Vec<Vec<bool>>;

fn run(input: &str, required_time_save: usize, skippable: usize) -> usize {
    let (start, end, mut maze) = parse_input(input);

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
    let (true_path, _) = dijkstra(start, |p| p.successors(&maze), |p| p == end).unwrap();

    let mut cheats: HashMap<usize, usize> = HashMap::new();
    let mut visited: HashSet<Pos> = HashSet::new();

    true_path.iter().enumerate().for_each(|(index, p)| {
        visited.insert(*p);

        let jumpable_targets = p.can_jump_to(&true_path, &skippable);

        for (target, distance) in jumpable_targets {
            if !visited.contains(&target) {
                let start_index = true_path.iter().position(|p| p == &target).unwrap();

                let saved = start_index - index - distance;

                cheats.entry(saved).or_insert(0);
                cheats.entry(saved).and_modify(|count| {
                    *count += 1;
                });
            }
        }
    });

    cheats
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

    fn can_jump_to(&self, path: &Vec<Pos>, skippable: &usize) -> HashSet<(Pos, usize)> {
        let &Pos(x, y) = self;

        let mut jumpable_positions: HashSet<(Pos, usize)> = HashSet::new();
        let current_index = path.iter().position(|p| p == self).unwrap();

        for pos in path[current_index..].iter() {
            let dx = x.abs_diff(pos.0);
            let dy = y.abs_diff(pos.1);

            let distance = dx + dy;
            if pos == &path[current_index + distance] {
                continue;
            }

            if distance <= *skippable {
                jumpable_positions.insert((*pos, distance));
            }
        }

        jumpable_positions
    }
}

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

            print!(
                "{}",
                if jumpable_positions.contains(&Pos(x, y)) {
                    Colorize::red("J")
                } else {
                    Colorize::white(char_to_print)
                }
            );
        }
        print!("\n");
    }
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
