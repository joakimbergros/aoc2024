use std::{collections::HashMap, fmt::Display};

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Outcome {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Ascending => "ascending",
                Direction::Descending => "descending",
            }
        )
    }
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    let result = data
        .iter()
        .filter_map(|set| {
            let mut window = set.windows(2).peekable();

            let direction = match window.peek() {
                Some([left, right]) => match left > right {
                    true => Direction::Descending,
                    false => Direction::Ascending,
                },
                _ => unreachable!("Window should always have 2 items"),
            };

            while let Some([left, right]) = window.next() {
                let diff = left.abs_diff(*right);

                if !(1..=3).contains(&diff) {
                    return None;
                }

                if (direction == Direction::Descending && left < right)
                    || (direction == Direction::Ascending && left > right)
                {
                    return None;
                }
            }

            Some(Outcome::Safe)
        })
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let bounds = 1..=3;

    for locations in data {
        locations
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (index, i)| {
                let exists = acc.contains_key(&index);

                if !exists {
                    acc.insert(index, i);
                }

                println!("{}: {} is in: {}", index, i, exists);

                acc
            });
    }

    return None;

    let result: Vec<Outcome> = data
        .iter()
        .map(|set| {
            let mut previous_location = None;
            let mut extra_life_used = false;

            println!(
                "Handling set: {}",
                &set.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );

            let mut locations = set.windows(2).peekable();

            let peeked = &locations
                .peek()
                .expect("Should have 2 numbers at the start");
            let direction = match peeked[0] > peeked[1] {
                true => Direction::Descending,
                false => Direction::Ascending,
            };
            println!("We're {}", direction);

            while let Some(location) = locations.next() {
                let left = location.get(0).expect("Should have a left");
                let right = location.get(1).expect("Should have a right");

                let Some(prev) = previous_location else {
                    previous_location = Some(left);

                    println!(
                        "First item {}, saving to previous location",
                        previous_location.unwrap()
                    );

                    continue;
                };

                println!("Comparing: {} and {}", prev, left);

                let diff = prev.abs_diff(*left);
                println!("Diff between {} and {} is {}", prev, left, diff);

                if bounds.contains(&diff) {
                    println!("We're within limit of 3");

                    previous_location = Some(left);

                    continue;
                }

                let Some(peek) = locations.peek() else {
                    return Outcome::Unsafe;
                };

                let left_abs = prev.abs_diff(*right);
                let right_abs = left.abs_diff(peek[0]);

                if bounds.contains(&left_abs) && !extra_life_used {
                    println!("It fits if we skip {}", left);

                    extra_life_used = true;
                    previous_location = Some(right);

                    continue;
                }

                if bounds.contains(&right_abs) && !extra_life_used {
                    println!("It fits if we skip {} in next pair", peek[0]);

                    extra_life_used = true;
                    previous_location = peek.get(0);
                    locations.next();

                    continue;
                }

                println!("********* The next blocks doesn't fit either, it's unsafe! **********");

                return Outcome::Unsafe;
            }

            println!("Every location are within limits!");
            println!("=================================");

            Outcome::Safe
        })
        .collect();

    Some(
        result
            .iter()
            .filter(|outcome| **outcome == Outcome::Safe)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
