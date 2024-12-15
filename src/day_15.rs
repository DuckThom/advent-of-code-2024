pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

use crossterm::{cursor::Hide, cursor::MoveTo, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;

fn part1(input: &str) -> usize {
    let (mut player, mut grid, directions) = parse_input(input, false);

    run_sokoban(&mut player, &mut grid, &directions);

    get_score(&grid)
}

fn part2(input: &str) -> usize {
    let (mut player, mut grid, directions) = parse_input(input, true);

    run_sokoban(&mut player, &mut grid, &directions);

    get_score(&grid)
}

#[derive(Debug, PartialEq)]
enum BlockType {
    Wall,
    Box,
    Player,
}

#[derive(Debug)]
struct Block {
    block_type: BlockType,
}

fn parse_input(input: &str, wide: bool) -> ((usize, usize), Vec<Vec<Option<Block>>>, Vec<(isize, isize)>) {
    let mut player: Option<(usize, usize)> = None;
    let mut grid: Vec<Vec<Option<Block>>> = vec![];
    let mut directions: Vec<(isize, isize)> = vec![];
    let mut parse_type = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        if line.is_empty() {
            parse_type = 1;

            return;
        }

        if parse_type == 0 {
            let mut row: Vec<Option<Block>> = vec![];

            line.chars().enumerate().for_each(|(x, c)| {
                for _ in 0..(wide as usize) {
                    if c == '#' {
                        row.push(Some(Block {
                            block_type: BlockType::Wall,
                        }));
                    } else if c == '.' {
                        row.push(None);
                    } else if c == 'O' {
                        row.push(Some(Block {
                            block_type: BlockType::Box,
                        }));
                    } else if c == '@' {
                        row.push(Some(Block {
                            block_type: BlockType::Player,
                        }));

                        if player.is_none() {
                            player = Some((x, y));
                        }
                    }
                }
            });

            grid.push(row);

            return;
        }

        if parse_type == 1 {
            line.chars().for_each(|c| {
                if c == '<' {
                    directions.push((-1, 0));
                } else if c == '^' {
                    directions.push((0, -1));
                } else if c == '>' {
                    directions.push((1, 0));
                } else if c == 'v' {
                    directions.push((0, 1));
                }
            });
        }
    });

    (player.unwrap(), grid, directions)
}

fn run_sokoban(
    player: &mut (usize, usize),
    grid: &mut Vec<Vec<Option<Block>>>,
    directions: &Vec<(isize, isize)>,
) {
    for direction in directions {
        step(player, grid, direction);

        // print_grid(grid, direction);
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Option<Block>>>, direction: &(isize, isize)) {
    let mut out = stdout();

    out.queue(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))
    .unwrap();
    out.queue(Hide).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();

    grid.iter().for_each(|row| {
        row.iter().for_each(|block| {
            if block.is_none() {
                out.write(" ".as_bytes()).unwrap();
            } else {
                let b = block.as_ref().unwrap();

                if b.block_type == BlockType::Box {
                    out.write("█".as_bytes()).unwrap();
                } else if b.block_type == BlockType::Player {
                    out.write("@".as_bytes()).unwrap();
                } else if b.block_type == BlockType::Wall {
                    out.write("#".as_bytes()).unwrap();
                }
            }
        });

        out.write("\n".as_bytes()).unwrap();
    });

    out.write("\n".as_bytes()).unwrap();

    if direction == &(0, -1) {
        out.write("v".as_bytes()).unwrap();
    } else if direction == &(0, 1) {
        out.write("^".as_bytes()).unwrap();
    } else if direction == &(-1, 0) {
        out.write("<".as_bytes()).unwrap();
    } else if direction == &(1, 0) {
        out.write(">".as_bytes()).unwrap();
    }

    out.write("\n".as_bytes()).unwrap();

    out.flush().unwrap();
    sleep(std::time::Duration::from_millis(10));
}

fn step(
    player: &mut (usize, usize),
    grid: &mut Vec<Vec<Option<Block>>>,
    direction: &(isize, isize),
) {
    let mut blocks_to_move: Vec<(usize, usize)> = vec![player.clone()];
    let mut pointer: (usize, usize) = (player.0, player.1);

    loop {
        let next = (
            pointer.0.overflowing_add_signed(direction.0).0,
            pointer.1.overflowing_add_signed(direction.1).0,
        );

        let next_block = grid[next.1][next.0].as_ref();

        if next_block.is_none() {
            blocks_to_move.push(next);

            break;
        }

        if next_block.unwrap().block_type == BlockType::Wall {
            blocks_to_move.clear();

            break;
        } else if next_block.unwrap().block_type == BlockType::Box {
            blocks_to_move.push(next);

            pointer = next;
        }
    }

    if !blocks_to_move.is_empty() {
        for i in (1..blocks_to_move.len()).rev() {
            swap(grid, &blocks_to_move[i], &blocks_to_move[i - 1]);
        }

        player.0 = player.0.overflowing_add_signed(direction.0).0;
        player.1 = player.1.overflowing_add_signed(direction.1).0;
    }
}

fn swap(grid: &mut Vec<Vec<Option<Block>>>, a: &(usize, usize), b: &(usize, usize)) {
    let temp = grid[a.1][a.0].take();
    grid[a.1][a.0] = grid[b.1][b.0].take();
    grid[b.1][b.0] = temp;
}

fn get_score(grid: &Vec<Vec<Option<Block>>>) -> usize {
    let mut score: usize = 0;

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, block)| {
            if block.is_some() && block.as_ref().unwrap().block_type == BlockType::Box {
                score += (y * 100) + x;
            }
        })
    });

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_TEST_INPUT: &str = include_str!("../inputs/day_15/small_test");
    const LARGE_TEST_INPUT: &str = include_str!("../inputs/day_15/large_test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(SMALL_TEST_INPUT), 2028);
        assert_eq!(part1(LARGE_TEST_INPUT), 10092);
    }

    #[test]
    pub fn test_part2() {
        // assert_eq!(part1(SMALL_TEST_INPUT), 2028);
        assert_eq!(part2(LARGE_TEST_INPUT), 9021);
    }
}
