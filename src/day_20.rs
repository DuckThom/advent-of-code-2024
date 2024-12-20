use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input, 100));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &str, required_time_save: usize) -> usize {
    let (start, end, mut maze) = parse_input(input);

    block_dead_ends(&mut maze, &start, &end);

    let cheats: HashMap<usize, usize> = cheat_the_maze(&mut maze, &start, &end);

    count_cheats(&cheats, &required_time_save)
}

// fn part2(input: &str) -> usize {
//
// }

fn count_cheats(cheats: &HashMap<usize, usize>, required_time_save: &usize) -> usize {
    cheats.into_iter().fold(0, |acc, (score, count)| {
        if score >= required_time_save {
            return acc + count;
        }

        acc
    })
}

fn cheat_the_maze(maze: &Vec<Vec<bool>>, start: &Pos, end: &Pos) -> HashMap<usize, usize> {
    let (_, score) = do_the_dijkstra_thing(&maze, &start, &end).unwrap();
    let mut cheats: HashMap<usize, usize> = HashMap::new();

    maze.iter().enumerate().for_each(|(y, row)| {
        if y == 0 || y == maze.len() - 1 {
            return;
        }

        row.iter().enumerate().for_each(|(x, cell)| {
            if x == 0 || x == row.len() - 1 {
                return;
            }

            if !cell {
                let mut cheated_maze = maze.clone();
                cheated_maze[y][x] = true;

                let (_, cheated_score) =
                    do_the_dijkstra_thing(&cheated_maze, &start, &end)
                        .unwrap();

                if cheated_score < score {
                    cheats.entry(score - cheated_score).or_insert(0);
                    cheats.entry(score - cheated_score).and_modify(|count| {
                        *count += 1;
                    });
                }
            }
        })
    });

    cheats
}

fn do_the_dijkstra_thing(
    maze: &Vec<Vec<bool>>,
    start: &Pos,
    end: &Pos,
) -> Option<(Vec<Pos>, usize)> {
    dijkstra(start, |p| p.successors(&maze), |p| p == end)
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

impl Pos {
    fn successors(&self, maze: &Vec<Vec<bool>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut next: Vec<Pos> = vec![];

        if y > 0 && maze[y - 1][x] {
            next.push(Pos(x, y - 1));
        }

        if maze[y + 1][x] {
            next.push(Pos(x, y + 1));
        }

        if x > 0 && maze[y][x - 1] {
            next.push(Pos(x - 1, y));
        }

        if x < maze[y].len() - 1 && maze[y][x + 1]{
            next.push(Pos(x + 1, y));
        }

        next.into_iter().map(|p| (p, 1)).collect()
    }
}

fn print_maze(maze: &Vec<Vec<bool>>, start: &Pos, end: &Pos, path: &Vec<Pos>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *start == Pos(x, y) {
                print!("S");
            } else if *end == Pos(x, y) {
                print!("E");
            } else {
                if path.iter().find(|p| **p == Pos(x, y)).is_some() {
                    print!("*");
                } else {
                    print!("{}", if *cell { '.' } else { '█' });
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

    const TEST_INPUT: &str = include_str!("../inputs/day_20/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 10);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(SMALL_TEST_INPUT), 45);
    //     assert_eq!(part2(LARGE_TEST_INPUT), 64);
    // }
}
