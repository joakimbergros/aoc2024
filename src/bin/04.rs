use std::{collections::HashMap, iter::{self, Filter}};

use itertools::Itertools;

advent_of_code::solution!(4);

const EXPECTED_WORDS: [char; 4] = ['X', 'M', 'A', 'S'];
const NEIGHBOUR_MAP: [(isize, isize); 9] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1),
];

type Coordinate = (usize, usize);
type Map = HashMap<Coordinate, char>;

trait Mappable {
    fn scan(&self, coordinate: &Coordinate, expected: char) -> u32;
}

impl Mappable for Map {
    fn scan(&self, coordinate: &Coordinate, expected: char) -> u32 {
        let mut appearances = 0;

        for (x, y) in NEIGHBOUR_MAP {
            let fx=  coordinate.0 as isize + x;
            let fy=  coordinate.1 as isize + y;

            if fx.is_negative() || fy.is_negative() {
                continue;
            }

            let next_coordinate = (fx as usize, fy as usize);

            let letters = match self.get(&next_coordinate) {
                Some(v) if *v == expected => self.scan(&next_coordinate, expected),
                _ => continue,
            };

            if letters != 3 {
                continue;
            }

            appearances += 1;
        }

        appearances
    }
}

fn parse(input: &str) -> HashMap<Coordinate, char> {
    let mut map: HashMap<Coordinate, char> = HashMap::new();

    for (li, lv) in input.lines().enumerate() {
        for (ci, cv) in lv.chars().enumerate() {
            map.insert((li, ci), cv);
        }
    }
    
    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut total = 0;

    for (coordinate, c) in &map {
        if *c != 'X' {
            continue;
        }

        total += map.scan(coordinate, 'M');

        /* for (x, y) in NEIGHBOUR_MAP {
            let fx=  coordinate.0 as isize + x;
            let fy=  coordinate.1 as isize + y;

            if fx.is_negative() || fy.is_negative() {
                continue;
            }

            let val = match map.get(&(fx as usize, fy as usize)) {
                Some(v) if EXPECTED_WORDS.contains(v) => v,
                _ => continue,
            };

            
        } */
    }

    Some(total)
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
