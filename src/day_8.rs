use crate::utils;
use std::collections::{HashMap, HashSet};

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(PartialEq, Debug)]
struct Node {
    id: char,
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct AntiNode {
    id: char,
    x: i32,
    y: i32,
}

fn part1(input: &str) -> usize {
    let map = utils::input_to_char_matrix(&input);

    let nodes = get_node_locations(&map);
    let antinodes = calculate_antinodes(&map, &nodes, false);

    print_map(&map, &antinodes);

    unique_antinodes(&antinodes).len()
}

fn part2(input: &str) -> usize {
    let map = utils::input_to_char_matrix(&input);

    let nodes = get_node_locations(&map);
    let antinodes = calculate_antinodes(&map, &nodes, true);

    print_map(&map, &antinodes);

    unique_antinodes(&antinodes).len()
}

fn print_map(map: &Vec<Vec<char>>, antinodes: &HashSet<AntiNode>) {
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            let antinode = antinodes
                .iter()
                .find(|n| n.x == x as i32 && n.y == y as i32);

            if antinode.is_some() {
                print!("({})", *c);
            } else {
                print!(" {} ", *c)
            }
        });

        print!("\n")
    });
}

fn get_node_locations(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Node>> {
    let mut locations = HashMap::new();

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c != '.' {
                let node_locations: &mut Vec<Node> = locations.entry(*c).or_insert_with(Vec::new);

                node_locations.push(Node {
                    id: *c,
                    x: x as i32,
                    y: y as i32,
                });
            }
        })
    });

    locations
}

fn unique_antinodes(antinodes: &HashSet<AntiNode>) -> HashSet<(i32, i32)> {
    antinodes
        .iter()
        .map(|antinode| (antinode.x, antinode.y))
        .collect()
}

fn calculate_antinodes(
    map: &Vec<Vec<char>>,
    nodes: &HashMap<char, Vec<Node>>,
    with_harmonics: bool,
) -> HashSet<AntiNode> {
    let mut antinodes = HashSet::new();

    for (_, node_group) in nodes.iter() {
        for node in node_group {
            node_group.iter().for_each(|inner_node| {
                if inner_node == node {
                    return;
                }

                let x_diff: i32 = node.x - inner_node.x;
                let y_diff: i32 = node.y - inner_node.y;

                if x_diff == 0 && y_diff == 0 {
                    return;
                }

                if with_harmonics {
                    let mut new_x: i32 = node.x;
                    let mut new_y: i32 = node.y;

                    loop {
                        new_x += x_diff;
                        new_y += y_diff;

                        let antinode = AntiNode {
                            id: node.id,
                            x: new_x,
                            y: new_y,
                        };

                        if is_out_of_bounds(&map, &antinode) {
                            break;
                        }

                        antinodes.insert(antinode);
                    }

                    new_x = node.x;
                    new_y = node.y;

                    loop {
                        new_x -= x_diff;
                        new_y -= y_diff;

                        let antinode = AntiNode {
                            id: node.id,
                            x: new_x,
                            y: new_y,
                        };

                        if is_out_of_bounds(&map, &antinode) {
                            break;
                        }

                        antinodes.insert(antinode);
                    }
                } else {
                    let antinode1 = AntiNode {
                        id: node.id,
                        x: node.x + x_diff,
                        y: node.y + y_diff,
                    };

                    let antinode2 = AntiNode {
                        id: node.id,
                        x: node.x - x_diff,
                        y: node.y - y_diff,
                    };

                    if !is_out_of_bounds(&map, &antinode1) && !overlaps_node(&node, &antinode1) {
                        antinodes.insert(antinode1);
                    }

                    if !is_out_of_bounds(&map, &antinode2)
                        && !overlaps_node(&inner_node, &antinode2)
                    {
                        antinodes.insert(antinode2);
                    }
                }
            })
        }
    }

    antinodes
}

fn is_out_of_bounds(map: &Vec<Vec<char>>, antinode: &AntiNode) -> bool {
    antinode.x < 0
        || antinode.y < 0
        || antinode.x >= map[0].len() as i32
        || antinode.y >= map.len() as i32
}

fn overlaps_node(current_node: &Node, antinode: &AntiNode) -> bool {
    current_node.x == antinode.x && current_node.y == antinode.y
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_8/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 34);
    }
}
