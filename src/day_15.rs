use std::collections::HashSet;

use crossterm::{cursor::MoveTo, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut sokoban = parse_input(input, false);

    sokoban.run()
}

fn part2(input: &str) -> usize {
    let mut sokoban = parse_input(input, true);

    sokoban.run()
}

type Direction = (isize, isize);
type Position = (usize, usize);

#[derive(Clone, PartialEq, Hash, Eq)]
struct MovableTile {
    locations: Vec<Position>,
}

impl MovableTile {
    fn can_move(&self, walls: &Vec<Position>, direction: &Direction) -> bool {
        self.locations.iter().all(|location| {
            let next = (
                location.0.overflowing_add_signed(direction.0).0,
                location.1.overflowing_add_signed(direction.1).0,
            );

            !walls.contains(&next)
        })
    }

    fn get_next(&self, boxes: &Vec<MovableTile>, direction: &Direction) -> HashSet<usize> {
        let mut next_boxes: HashSet<usize> = HashSet::new();

        for location in self.locations.iter() {
            let next = (
                location.0.overflowing_add_signed(direction.0).0,
                location.1.overflowing_add_signed(direction.1).0,
            );

            if let Some(next_box) = boxes.iter().position(|b| b.locations.contains(&next)) {
                next_boxes.insert(next_box);
            }
        }

        next_boxes
    }

    fn move_to(&mut self, direction: &Direction) {
        self.locations = self
            .locations
            .iter()
            .map(|(x, y)| {
                (
                    x.overflowing_add_signed(direction.0).0,
                    y.overflowing_add_signed(direction.1).0,
                )
            })
            .collect();
    }
}

struct Sokoban {
    player: MovableTile,
    boxes: Vec<MovableTile>,
    walls: Vec<Position>,
    directions: Vec<Direction>,
}

impl Sokoban {
    fn run(&mut self) -> usize {
        for direction in self.directions.clone() {
            self.step(&direction);

            #[cfg(test)]
            self.render(&self.player, &self.boxes, &self.walls, &direction);
        }

        self.get_score()
    }

    fn get_score(&self) -> usize {
        self.boxes.iter().fold(0, |acc, tile| {
            let (x, y) = tile.locations.first().unwrap();

            acc + ((*y * 100) + *x)
        })
    }

    fn step(&mut self, direction: &Direction) {
        let old_boxes = self.boxes.clone();
        let old_player = self.player.clone();

        let mut blocks_to_process: Vec<usize> = vec![usize::MAX];
        let mut hit_wall = false;
        let mut processed: HashSet<usize> = HashSet::new();

        'outer: while !blocks_to_process.is_empty() && !hit_wall {
            let tile_index = blocks_to_process.pop().unwrap();
            let tile = if tile_index == usize::MAX {
                &mut self.player
            } else {
                &mut self.boxes[tile_index]
            };

            processed.insert(tile_index);

            if !tile.can_move(&self.walls, direction) {
                hit_wall = true;
                break 'outer;
            }

            for index in tile.get_next(&old_boxes, &direction) {
                if !blocks_to_process.contains(&index) && !processed.contains(&index) {
                    blocks_to_process.push(index);
                }
            }

            tile.move_to(&direction);
        }

        if hit_wall {
            self.boxes = old_boxes;
            self.player = old_player;
        }
    }

    #[allow(dead_code)]
    fn render(
        &self,
        player: &MovableTile,
        boxes: &Vec<MovableTile>,
        walls: &Vec<Position>,
        direction: &Direction,
    ) {
        let mut out = stdout();

        out.queue(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();
        out.queue(MoveTo(0, 0)).unwrap();

        if direction.1 == -1 {
            out.write(b"^").unwrap();
        } else if direction.1 == 1 {
            out.write(b"v").unwrap();
        } else if direction.0 == -1 {
            out.write(b"<").unwrap();
        } else if direction.0 == 1 {
            out.write(b">").unwrap();
        }
        out.write("\n".as_bytes()).unwrap();

        for y in 0..=*walls.iter().map(|(_, y)| y).max().unwrap() {
            for x in 0..=*walls.iter().map(|(x, _)| x).max().unwrap() {
                if walls.contains(&(x, y)) {
                    out.write(b"#").unwrap();
                } else if player.locations.contains(&(x, y)) {
                    out.write(b"@").unwrap();
                } else if boxes
                    .iter()
                    .any(|tile| tile.locations.first().unwrap() == &(x, y))
                {
                    out.write("[".as_bytes()).unwrap();
                } else if boxes
                    .iter()
                    .any(|tile| tile.locations.last().unwrap() == &(x, y))
                {
                    out.write("]".as_bytes()).unwrap();
                } else {
                    out.write(b" ").unwrap();
                }
            }

            out.write("\n".as_bytes()).unwrap();
        }

        out.write("\n".as_bytes()).unwrap();
        out.write("\n".as_bytes()).unwrap();

        out.flush().unwrap();

        sleep(std::time::Duration::from_millis(1000));
    }
}

fn parse_input(input: &str, wide: bool) -> Sokoban {
    let mut player: MovableTile = MovableTile { locations: vec![] };
    let mut boxes: Vec<MovableTile> = vec![];
    let mut walls: Vec<Position> = vec![];
    let mut directions: Vec<Direction> = vec![];
    let mut parse_type = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        let mut x = 0;

        if line.is_empty() {
            parse_type = 1;

            return;
        }

        if parse_type == 0 {
            line.chars().for_each(|c| {
                if c == '.' {
                    x += if wide { 2 } else { 1 };

                    return;
                }

                if c == '#' {
                    walls.push((x, y));
                    x += 1;

                    if wide {
                        walls.push((x, y));
                        x += 1;
                    }
                } else if c == 'O' {
                    let mut locations = vec![(x, y)];
                    x += 1;
                    if wide {
                        locations.push((x, y));
                        x += 1;
                    }

                    boxes.push(MovableTile { locations });
                } else if c == '@' {
                    player = MovableTile {
                        locations: vec![(x, y)],
                    };

                    x += if wide { 2 } else { 1 };
                }
            });

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

    Sokoban {
        player,
        boxes,
        walls,
        directions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_TEST_INPUT: &str = include_str!("../inputs/day_15/small_test");
    const LARGE_TEST_INPUT: &str = include_str!("../inputs/day_15/large_test");
    const ANOTHER_TEST_INPUT: &str = include_str!("../inputs/day_15/another_test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(SMALL_TEST_INPUT), 2028);
        assert_eq!(part1(LARGE_TEST_INPUT), 10092);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(ANOTHER_TEST_INPUT), 11042);
        assert_eq!(part2(LARGE_TEST_INPUT), 9021);
    }
}
