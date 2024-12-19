use std::collections::HashSet;

use crossterm::{cursor::Hide, cursor::MoveTo, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;

pub fn execute(input: &str) {
    // println!("Part 1: {}", part1(&input));
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

#[derive(Clone, PartialEq)]
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
            // self.render();
        }
        self.render();
        self.get_score()
    }

    fn get_score(&self) -> usize {
        self.boxes.iter().fold(0, |acc, tile| {
            let (x, y) = tile.locations.first().unwrap();

            acc + ((*y * 100) + *x)
        })
    }

    fn step(&mut self, direction: &Direction) {
        let mut new_boxes = self.boxes.clone();
        let mut new_player = self.player.clone();

        let mut blocks_to_process: Vec<MovableTile> = vec![self.player.clone()];
        let mut abort = false;

        'outer: while !blocks_to_process.is_empty() && !abort {
            let tile = blocks_to_process.pop().unwrap();

            if tile.can_move(&self.walls, direction) {
                if tile == self.player {
                    new_player.move_to(&direction);
                } else {
                    let index = self.boxes.iter().position(|t| t == &tile).unwrap();

                    new_boxes[index].move_to(&direction);
                }

                let locations = tile.locations.clone();
                for location in tile.locations {
                    let next = (
                        location.0.overflowing_add_signed(direction.0).0,
                        location.1.overflowing_add_signed(direction.1).0,
                    );

                    if self.walls.contains(&next) {
                        abort = true;
                        break 'outer;
                    }

                    let matching_boxes: HashSet<usize> =
                        self.boxes
                            .iter()
                            .enumerate()
                            .fold(HashSet::new(), |mut acc, (index, t)| {
                                if t.locations.contains(&next) && !locations.contains(&next) {
                                    acc.insert(index);
                                }

                                acc
                            });

                    for i in matching_boxes {
                        if !blocks_to_process.contains(&self.boxes[i]) {
                            blocks_to_process.push(self.boxes[i].clone())
                        }
                    }
                }
            } else {
                blocks_to_process.clear();

                abort = true;
            }
        }

        if !abort {
            self.boxes = new_boxes;
            self.player = new_player;
        }
    }

    #[allow(dead_code)]
    fn render(&self) {
        let mut out = stdout();

        out.queue(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();
        out.queue(Hide).unwrap();
        out.queue(MoveTo(0, 0)).unwrap();

        for y in 0..=self
            .walls
            .iter()
            .max_by(|(_, ay), (_, by)| ay.cmp(by))
            .unwrap()
            .1
        {
            for x in 0..=*self
                .walls
                .iter()
                .map(|(x, _)| x)
                .max()
                .unwrap()
            {
                if self.walls.contains(&(x, y)) {
                    out.write(b"#").unwrap();
                } else if self.player.locations.contains(&(x, y)) {
                    out.write(b"@").unwrap();
                } else if self
                    .boxes
                    .iter()
                    .any(|tile| tile.locations.contains(&(x, y)))
                {
                    out.write("█".as_bytes()).unwrap();
                } else {
                    out.write(b" ").unwrap();
                }
            }

            out.write("\n".as_bytes()).unwrap();
        }

        out.flush().unwrap();
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
                    let locations = vec![(x, y)];
                    x += 2;

                    player = MovableTile { locations }
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

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(SMALL_TEST_INPUT), 2028);
        assert_eq!(part1(LARGE_TEST_INPUT), 10092);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(LARGE_TEST_INPUT), 9021);
    }
}
