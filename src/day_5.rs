use std::collections::HashMap;

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let (pages, orders) = parse_input(input);

    orders.iter().fold(0, |acc, order| {
        acc + if is_correct_order(&order, &pages).is_ok() {
            order[order.len() / 2]
        } else {
            0
        }
    })
}

fn part2(input: &str) -> usize {
    let (pages, orders) = parse_input(input);

    orders.iter().fold(0, |acc, order| {
        acc + if is_correct_order(&order, &pages).is_err() {
            let new_order = fix_order(&order, &pages);

            new_order[new_order.len() / 2] as usize
        } else {
            0
        }
    })
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut pages: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut orders: Vec<Vec<i32>> = vec![];
    let mut switch: bool = false;

    for line in input.lines() {
        if line.is_empty() {
            switch = true;
            continue;
        }

        if !switch {
            let parts: Vec<i32> = line.split("|").map(|s| s.parse().unwrap()).collect();

            if pages.contains_key(&parts[0]) {
                let i = pages.get_mut(&parts[0]).unwrap();

                i.push(parts[1].clone());
            } else {
                pages.insert(parts[0], vec![parts[1]]);
            }
        } else {
            let parts: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();

            orders.push(parts);
        }
    }

    (pages, orders)
}

fn is_correct_order(order: &Vec<i32>, pages: &HashMap<i32, Vec<i32>>) -> Result<bool, usize> {
    let mut pages_printed: Vec<i32> = vec![];

    for i in 0..order.len() {
        let page = &order[i];
        let page_deps: Option<&Vec<i32>> = pages.get(page);

        if page_deps.is_none() {
            pages_printed.push(*page);

            continue;
        }

        for dep in page_deps.unwrap() {
            if pages_printed.contains(dep) {
                return Err(i);
            }
        }

        pages_printed.push(*page);
    }

    Ok(true)
}

fn fix_order(order: &Vec<i32>, pages: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut new_order: Vec<i32> = order.clone();

    loop {
        let result = is_correct_order(&new_order, pages);
        if result.is_ok() {
            break;
        }

        new_order.swap(result.unwrap_err(), result.unwrap_err() - 1);
    }

    new_order
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_5/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}
