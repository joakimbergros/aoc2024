use itertools::Itertools;
use std::{fmt::Display, ops::RangeInclusive};

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Outcome {
    Safe,
    Unsafe,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Outcome::Safe => "safe",
                Outcome::Unsafe => "unsafe",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
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

fn parse(input: &str) -> Vec<Vec<i32>> {
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

const BOUNDS: RangeInclusive<i32> = 1..=3;

fn check_safety(locations: &Vec<i32>) -> Outcome {
    let mut direction: Option<Direction> = None;

    for (left, right) in locations.iter().tuple_windows() {
        let diff = left - right;

        match diff.signum() {
            1 => {
                let dir = direction.get_or_insert(Direction::Descending);
                if dir != &mut Direction::Descending {
                    return Outcome::Unsafe;
                }

                if !BOUNDS.contains(&diff) {
                    return Outcome::Unsafe;
                }
            }
            -1 => {
                let dir = direction.get_or_insert(Direction::Ascending);
                if dir != &mut Direction::Ascending {
                    return Outcome::Unsafe;
                }

                if !BOUNDS.contains(&diff.abs()) {
                    return Outcome::Unsafe;
                }
            }
            0 => {
                return Outcome::Unsafe;
            }
            _ => panic!("Should not come here"),
        }
    }

    Outcome::Safe
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let mut successful = 0;

    for locations in data {
        let outcome = check_safety(&locations);

        if outcome == Outcome::Safe {
            successful += 1;

            continue;
        }

        let mut second_chance_works = false;
        for i in 0..locations.len() {
            let mut clone = locations.clone();

            clone.remove(i);

            let second_try = check_safety(&clone);
            if second_try == Outcome::Safe {
                second_chance_works = true;
                break;
            }
        }

        if !second_chance_works {
            continue;
        }

        successful += 1;
    }

    Some(successful)
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
        assert_eq!(result, Some(4));
    }
}
