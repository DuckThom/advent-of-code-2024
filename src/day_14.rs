use regex::Regex;

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input, 103, 101));
    println!("Part 2: {}", part2(&input, 103, 101));
}

fn part1(input: &str, rows: i32, cols: i32) -> usize {
    let mut robots = parse_input(input);

    move_robots(&mut robots, 100, rows, cols);

    get_safety_factor(&robots, rows, cols)
}

fn part2(input: &str, rows: i32, cols: i32) -> usize {
    let mut robots = parse_input(input);
    let mut seconds: usize = 1;

    loop {
        move_robots(&mut robots, 1, rows, cols);

        if could_be_a_tree(&robots) {
            return seconds;
        }

        seconds += 1;
    }
}

fn could_be_a_tree(robots: &Vec<Robot>) -> bool {
    for robot in robots.iter() {
        let mut count = 1; // Start by counting the current robot
        let mut x = robot.location.0;
        let mut y = robot.location.1;

        for _ in 0..10 {
            x -= 1; // Move one step left
            y += 1; // Move one step down
            if robots.iter().any(|r| r.location == (x, y)) {
                count += 1;
            } else {
                break;
            }
        }

        if count == 10 {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn print_tree(robots: &Vec<Robot>, rows: i32, cols: i32, i: usize) {
    let mut tree: Vec<Vec<char>> = vec![vec!['.'; cols as usize]; rows as usize];

    use std::fs::File;
    use std::io::Write;

    robots.iter().for_each(|robot| {
        let (x, y) = robot.location;
        tree[y as usize][x as usize] = '#'; // Mark robot locations
    });

    // Construct the tree visualization as a string
    let mut tree_output = String::new();
    for row in &tree {
        tree_output.push_str(&format!("{}\n", row.iter().collect::<String>()));
    }

    // Write the output to a file
    let mut file = File::create(format!("output{}.txt", i)).expect("Unable to create file");
    file.write_all(tree_output.as_bytes())
        .expect("Unable to write data");
}

#[derive(Debug, Clone)]
struct Robot {
    location: (i32, i32),
    vector: (i32, i32),
}

fn parse_input(input: &str) -> Vec<Robot> {
    let matcher: Regex = Regex::new("^p=(\\d+),(\\d+)\\s+v=([-\\d]+),([-\\d]+)$").unwrap();

    let mut robots: Vec<Robot> = vec![];

    input.lines().for_each(|line| {
        let matches = matcher.captures(line).unwrap();

        let pos_x = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let pos_y = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vec_x = matches.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vec_y = matches.get(4).unwrap().as_str().parse::<i32>().unwrap();

        robots.push(Robot {
            location: (pos_x, pos_y),
            vector: (vec_x, vec_y),
        });
    });

    robots
}

fn wrap(value: i32, max: i32) -> i32 {
    ((value % max) + max) % max
}

fn move_robots(robots: &mut Vec<Robot>, steps: i32, rows: i32, cols: i32) {
    robots.iter_mut().for_each(|robot| {
        let original_x = robot.location.0;
        let original_y = robot.location.1;

        let vec_x = robot.vector.0;
        let vec_y = robot.vector.1;

        let new_x = original_x + (steps * vec_x);
        let new_y = original_y + (steps * vec_y);

        // Wrap positions within bounds
        robot.location.0 = wrap(new_x, cols); // Wrap x-coordinate
        robot.location.1 = wrap(new_y, rows); // Wrap y-coordinate
    });
}

fn get_safety_factor(robots: &Vec<Robot>, rows: i32, cols: i32) -> usize {
    let mut quadrants: [Vec<&Robot>; 4] = Default::default();

    robots.iter().for_each(|robot| {
        let (x, y) = robot.location;

        if x < cols / 2 && y < rows / 2 {
            quadrants[0].push(robot);
        } else if x > cols / 2 && y < rows / 2 {
            quadrants[1].push(robot);
        } else if x < cols / 2 && y > rows / 2 {
            quadrants[2].push(robot);
        } else if x > cols / 2 && y > rows / 2 {
            quadrants[3].push(robot);
        }
    });

    quadrants[0].len() * quadrants[1].len() * quadrants[2].len() * quadrants[3].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_14/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 7, 11), 12);
    }

    // #[test]
    // pub fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 1206);
    // }
}
