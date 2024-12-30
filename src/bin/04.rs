use std::{
    collections::HashMap,
    iter::{self, Filter},
};

use crate::Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};
use itertools::Itertools;

advent_of_code::solution!(4);

const EXPECTED_WORDS: [char; 4] = ['X', 'M', 'A', 'S'];
const WANTED_CHARS: [char; 3] = ['M', 'A', 'S'];
const NEIGHBOUR_MAP: [(isize, isize); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const NEIGHBOUR_MAP2: [Direction; 8] = [
    NorthWest(-1, -1),
    North(-1, 0),
    NorthEast(-1, 1),
    West(0, -1),
    East(0, 1),
    SouthWest(1, -1),
    South(1, 0),
    SouthEast(1, 1),
];

enum Direction {
    NorthWest(isize, isize),
    North(isize, isize),
    NorthEast(isize, isize),
    West(isize, isize),
    East(isize, isize),
    SouthWest(isize, isize),
    South(isize, isize),
    SouthEast(isize, isize),
}

impl Direction {
    fn from_coordinate(coord: Coordinate) -> Self {}
    fn values(&self) -> (&isize, &isize) {
        match self {
            NorthWest(x, y)
            | North(x, y)
            | NorthEast(x, y)
            | West(x, y)
            | East(x, y)
            | SouthWest(x, y)
            | South(x, y)
            | SouthEast(x, y) => (x, y),
        }
    }

    fn next(&self) -> Self {
        match self {
            NorthWest(x, y) => Direction::NorthWest(x - 1, y - 1),
            North(x, y) => Direction::North(x - 1, y.clone()),
            NorthEast(x, y) => Direction::NorthEast(x - 1, y + 1),
            West(x, y) => Direction::West(x.clone(), y - 1),
            East(x, y) => Direction::East(x.clone(), y + 1),
            SouthWest(x, y) => Direction::SouthWest(x + 1, y - 1),
            South(x, y) => Direction::South(x - 1, y.clone()),
            SouthEast(x, y) => Direction::SouthEast(x - 1, y + 1),
        }
    }
}

type Coordinate = (isize, isize);
type Map = HashMap<Coordinate, char>;

trait Mappable {
    fn scan(&self, coordinate: &Coordinate, index: usize) -> u32;
}

impl Mappable for Map {
    fn scan(&self, coordinate: &Coordinate, index: usize) -> u32 {
        if EXPECTED_WORDS.len() == index {
            return 0;
        }

        dbg!(&coordinate, EXPECTED_WORDS[index]);
        let mut appearances = 0;

        for (x, y) in NEIGHBOUR_MAP {
            let fx = coordinate.0 as isize + x;
            let fy = coordinate.1 as isize + y;

            if fx.is_negative() || fy.is_negative() {
                continue;
            }

            let next_coordinate = (fx, fy);

            let letters = match self.get(&next_coordinate) {
                Some(v) if *v == EXPECTED_WORDS[index] => {
                    dbg!(v);
                    self.scan(&next_coordinate, index + 1)
                }
                c => {
                    dbg!(c);
                    continue;
                }
            };

            dbg!(letters);

            appearances += 1;
        }

        appearances
    }
}

fn parse(input: &str) -> HashMap<Coordinate, char> {
    let mut map: HashMap<Coordinate, char> = HashMap::new();

    for (li, lv) in input.lines().enumerate() {
        for (ci, cv) in lv.chars().enumerate() {
            map.insert((li as isize, ci as isize), cv);
        }
    }

    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    //let mut total = 0;

    let result: u32 = map
        .iter()
        .filter(|(_, c)| **c == 'X')
        .map(|coord| {
            for direction in &NEIGHBOUR_MAP2 {
                direction.next()
            }

            for wanted in &WANTED_CHARS {}

            let mut start_char_index = 1;
            let mut total_words = 0;

            for dir in &NEIGHBOUR_MAP2 {
                let (dx, dy) = dir.values();
                let ((cx, cy), _) = coord;
                let fx = dx + cx;
                let fy = dy + cy;

                match map.get(&(fx, fy)) {
                    Some(letter) if *letter == EXPECTED_WORDS[start_char_index] => true,
                    _ => {
                        continue;
                    }
                };

                total_words += 1;

                /*let next_coordinate = (fx as usize, fy as usize);

                let letters = match self.get(&next_coordinate) {
                    Some(v) if *v == EXPECTED_WORDS[index] => {
                        dbg!(v);
                        self.scan(&next_coordinate, index + 1)
                    },
                    c => {
                        dbg!(c);
                        continue
                    },
                };

                dbg!(letters);

                appearances += 1;*/
            }

            total_words
        })
        .sum();

    dbg!(result);

    /*for (coordinate, c) in &map {
        if *c != 'X' {
            continue;
        }

        total += map.scan(coordinate, 0);

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
    }*/

    Some(123)
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
