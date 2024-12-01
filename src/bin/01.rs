use std::collections::HashMap;

advent_of_code::solution!(1);

const SPLIT_ARRAY_SIZE: usize = 2;

fn split_to_tuple(line: &str) -> (u32, u32) {
    let mut ids = line.split_whitespace();

    (
        ids.next()
            .expect("Data should have a left side")
            .parse()
            .expect("Data should be numeric"),
        ids.next()
            .expect("Data should have a right side")
            .parse()
            .expect("Data should be numeric"),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut right, mut left): (Vec<u32>, Vec<u32>) =
        input.lines().fold((vec![], vec![]), |mut acc, line| {
            let ids = split_to_tuple(line);

            acc.0.push(ids.0);
            acc.1.push(ids.1);

            acc
        });

    right.sort();
    left.sort();

    let result = right
        .iter()
        .zip(&left)
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .fold(HashMap::new(), |mut acc, line| {
            let ids = split_to_tuple(line);

            acc.entry(ids.0).and_modify(|e| *e += 1).or_insert(0_u32);
            acc.entry(ids.1).and_modify(|e| *e += 1).or_insert(0_u32);

            acc
        })
        .iter()
        .map(|(key, count)| key * count)
        .sum::<u32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
