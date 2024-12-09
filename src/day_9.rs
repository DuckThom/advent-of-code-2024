use std::str::FromStr;
use std::vec;
use crate::utils;

pub fn execute() {
    let input: String = utils::read_input_file(9);

    utils::print_day_banner(9);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let blocks = parse_input(input);
    let fragmented_blocks = fragment(&blocks);

    checksum(&fragmented_blocks)
}

fn part2(input: &str) -> usize {
    let blocks = parse_input(input);
    let defragmented_blocks = defragment(&blocks);

    checksum(&defragmented_blocks)
}

fn parse_input(input: &str) -> Vec<Option<usize>> {
    let mut disk = vec![];
    let bytes = input.as_bytes();

    let mut id = 0;

    for i in (0..bytes.len()).step_by(2) {
        let end = if i + 2 <= bytes.len() { i + 2 } else { bytes.len() };
        let chunk = &input[i..end].split_at(1);

        if !chunk.0.is_empty() {
            for _ in 0..usize::from_str(chunk.0).unwrap() {
                disk.push(Some(id));
            }
        }

        if !chunk.1.is_empty() {
            for _ in 0..usize::from_str(chunk.1).unwrap() {
                disk.push(None);
            }
        }

        id += 1;
    }

    disk
}

fn fragment(disk: &Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut new_disk: Vec<Option<usize>> = disk.clone();
    let mut pointer = 0;

    'outer: for i in (0..new_disk.len()).rev() {
        let char = new_disk[i];
        if char.is_none() {
            continue;
        }

        loop {
            if pointer >= i {
                break 'outer;
            }

            if new_disk[pointer].is_none() {
                new_disk.swap(i, pointer);
                pointer += 1;

                break;
            }

            pointer += 1
        }
    }

    new_disk.into_iter().collect()
}

fn move_chunk(buffer: &mut Vec<usize>, disk: &mut Vec<Option<usize>>, source: usize, target: usize) {
    for i in 0..buffer.len() {
        disk.swap(source + i, target + i);
    }
}

fn find_free_space(size: usize, disk: &Vec<Option<usize>>) -> Option<usize> {
    let mut consecutive_none_count = 0;
    let mut start_index = None;

    for (index, item) in disk.iter().enumerate() {
        if item.is_none() {
            if start_index.is_none() {
                start_index = Some(index);
            }

            consecutive_none_count += 1;
            if consecutive_none_count == size {
                return start_index;
            }
        } else {
            consecutive_none_count = 0;
            start_index = None;
        }
    }

    None
}

fn defragment(disk: &Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut new_disk: Vec<Option<usize>> = disk.clone();
    let mut current_id = disk.iter().max().unwrap().unwrap();

    loop {
        let mut buffer_end: Option<usize> = None;
        let mut buffer: Vec<usize> = vec![];

        for i in (0..new_disk.len()).rev() {
            let id = new_disk[i];
            if id.is_none() || current_id != id.unwrap() {
                continue;
            }

            buffer.push(id.unwrap());

            if buffer_end.is_none() {
                buffer_end = Some(i);
            }

            let next_char = if i == 0 {
                None
            } else {
                new_disk[i-1]
            };

            if buffer.len() > 0 && next_char.unwrap_or(current_id + 1) != current_id {
                let free_start = find_free_space(buffer.len(), &new_disk);

                // Cannot move
                if free_start.is_none() {
                    break;
                }

                let buffer_start = buffer_end.unwrap() - (buffer.len() - 1);

                if buffer_start < free_start.unwrap() {
                    break;
                }

                move_chunk(&mut buffer, &mut new_disk, buffer_start, free_start.unwrap());

                break;
            }
        }

        if current_id == 0 {
            break;
        }

        current_id -= 1;
    }

    new_disk.into_iter().collect()
}

fn checksum(disk: &Vec<Option<usize>>) -> usize {
    let mut checksum = 0;

    for i in 0..disk.len() {
        let id = disk[i];
        if id.is_none() {
            continue;
        }

        checksum += i * id.unwrap();
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_9/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2858);
    }
}
