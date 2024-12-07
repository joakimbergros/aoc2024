use std::iter::{self, Filter};

use itertools::Itertools;

advent_of_code::solution!(4);

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const MAX_NEIGHBOURS: u32 = 8;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forwards,
    Backwards,
}

trait Coordinated {
    fn get_char_by_coordinate(&self, x: u32, y: u32) -> Option<char>;
}

impl Coordinated for Vec<Vec<char>> {
    fn get_char_by_coordinate(&self, x: u32, y: u32) -> Option<char> {
        self.iter().enumerate().filter_map(|(i, xx)| {
            xx.get(y)?
        })
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => 'X',
                    'M' => 'M',
                    'A' => 'M',
                    'S' => 'S',
                    _ => '.',
                })
                .collect()
        })
        .collect()
}


}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    dbg!(map);

    for (x_idx, x_val) in map.iter().enumerate() {
        for (y_idx, y_val) in x_val.iter().enumerate() {

        }
    }

    //map.iter()
    //    .enumerate()
    //    .filter(|(x_index, x_val)| x_val.iter().enumerate().filter(|(y_index, y_val)| y_index));

    for (line_index, line) in input.lines().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            println!("line {line_index} char {c}:{char_index}");

            let direction = match c {
                'X' => Direction::Forwards,
                'S' => Direction::Backwards,
                _ => {
                    continue;
                }
            };

            for neighbour_index in 0..MAX_NEIGHBOURS {}
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
