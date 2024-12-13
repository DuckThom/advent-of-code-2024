use crate::utils;
use std::collections::HashMap;

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    calculate_price(input, count_fences)
}

fn part2(input: &str) -> usize {
    calculate_price(input, count_fence_edges)
}

fn calculate_price(input: &str, calculator: fn(&Vec<Vec<usize>>) -> HashMap<usize, usize>) -> usize {
    let plants = utils::input_to_char_matrix(input);
    let groups = create_char_groups(&plants);

    let plant_count = count_plants(&groups);
    let fence_count = calculator(&groups);

    plant_count.iter().fold(0, |acc, (plant, count)| {
        acc + count * fence_count.get(plant).unwrap_or(&0)
    })
}

fn count_plants(plants: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    plants.iter().fold(HashMap::new(), |mut acc, row| {
        for plant in row {
            *acc.entry(*plant).or_insert(0) += 1;
        }
        acc
    })
}

fn count_fences(plants: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    let mut fences: HashMap<usize, usize> = HashMap::new();

    for (y, row) in plants.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            *fences.entry(*plant).or_insert(0) += get_fence_count(x, y, plant, plants);
        }
    }

    fences
}

fn count_fence_edges(plants: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    let rows = plants.len() + 1;
    let cols = plants[0].len() + 1;

    // Fence grid, usize denotes adjacent plant group id's
    let mut fence_points: Vec<Vec<HashMap<usize, usize>>> = vec![vec![HashMap::new(); cols]; rows];

    for y in 0..rows {
        for x in 0..cols {
            let mut map: HashMap<usize, usize> = HashMap::new();
            let mut top_left: Option<usize> = None;
            let mut top_right: Option<usize> = None;
            let mut bot_left: Option<usize> = None;
            let mut bot_right: Option<usize> = None;

            if y > 0 && x > 0 {
                top_left = Some(plants[y - 1][x - 1]);

                *map.entry(plants[y - 1][x - 1]).or_insert(0usize) += 1;
            }

            if y > 0 && x < plants[0].len() {
                top_right = Some(plants[y - 1][x]);

                *map.entry(plants[y - 1][x]).or_insert(0usize) += 1;
            }

            if y < plants.len() && x > 0 {
                bot_left = Some(plants[y][x - 1]);

                *map.entry(plants[y][x - 1]).or_insert(0usize) += 1;
            }

            if y < plants.len() && x < plants[0].len() {
                bot_right = Some(plants[y][x]);

                *map.entry(plants[y][x]).or_insert(0usize) += 1;
            }

            // All 4 adjacent items are equal
            if top_left.is_some()
                && top_right.is_some()
                && bot_left.is_some()
                && bot_right.is_some()
            {
                if top_left == top_right && top_left == bot_left && top_left == bot_right {
                    fence_points[y][x] = map;

                    continue;
                }
            }

            // \ diagonal match
            if top_left.is_some()
                && top_left == bot_right
                && top_left != bot_left
                && top_left != top_right
            {
                let id = top_left.unwrap();

                *map.entry(id).or_insert(0usize) = 5;
            }

            // / diagonal match
            if top_right.is_some()
                && top_right == bot_left
                && top_right != bot_right
                && top_right != top_left
            {
                let id = top_right.unwrap();

                *map.entry(id).or_insert(0usize) = 5;
            }

            fence_points[y][x] = map;
        }
    }

    fence_points.iter().fold(HashMap::new(), |mut acc, row| {
        row.iter().for_each(|cell| {
            cell.iter().for_each(|(id, count)| {
                if *count == 5 {
                    *acc.entry(*id).or_insert(0) += 2;
                } else if *count == 1 || *count == 3 {
                    *acc.entry(*id).or_insert(0) += 1;
                }
            })
        });
        acc
    })
}

fn get_fence_count(x: usize, y: usize, plant: &usize, plants: &Vec<Vec<usize>>) -> usize {
    let mut count: usize = 4;

    if y > 0 && plants[y - 1][x] == *plant {
        count -= 1;
    }

    if y < plants.len() - 1 && plants[y + 1][x] == *plant {
        count -= 1;
    }

    if x > 0 && plants[y][x - 1] == *plant {
        count -= 1;
    }

    if x < plants[0].len() - 1 && plants[y][x + 1] == *plant {
        count -= 1;
    }

    count
}

fn create_char_groups(plants: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let rows = plants.len();
    let cols = plants[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut groups = vec![vec![0; cols]; rows];

    let mut current_group_id = 0;

    // Helper function to perform DFS
    fn depth_first_search(
        x: usize,
        y: usize,
        rows: usize,
        cols: usize,
        grid: &[Vec<char>],
        visited: &mut Vec<Vec<bool>>,
        groups: &mut Vec<Vec<usize>>,
        current_group_id: usize,
    ) {
        // Possible directions: up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        visited[y][x] = true;
        groups[y][x] = current_group_id;

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && nx < cols as isize
                && ny < rows as isize
                && !visited[ny as usize][nx as usize]
                && grid[ny as usize][nx as usize] == grid[y][x]
            {
                depth_first_search(
                    nx as usize,
                    ny as usize,
                    rows,
                    cols,
                    grid,
                    visited,
                    groups,
                    current_group_id,
                );
            }
        }
    }

    // Iterate through each cell in the grid
    for y in 0..rows {
        for x in 0..cols {
            if !visited[y][x] {
                current_group_id += 1;
                depth_first_search(
                    x,
                    y,
                    rows,
                    cols,
                    plants,
                    &mut visited,
                    &mut groups,
                    current_group_id,
                );
            }
        }
    }

    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_12/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1930);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1206);
    }
}
