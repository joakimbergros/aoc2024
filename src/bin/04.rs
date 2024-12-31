use std::{
    char::REPLACEMENT_CHARACTER, collections::HashMap, fmt::Display, ops::Add, thread::current,
};

use crate::Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};

advent_of_code::solution!(4);

const START_CHAR: char = 'X';
const START_CHAR_2: char = 'A';
const WANTED_CHARS: [char; 3] = ['M', 'A', 'S'];

type Map = HashMap<Coordinate, char>;

#[derive(Debug)]
enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl Direction {
    fn next(&self, c: &Coordinate) -> Coordinate {
        c.from_direction(&self)
    }

    const VALUES: [Self; 8] = [
        Self::NorthWest,
        Self::North,
        Self::NorthEast,
        Self::West,
        Self::East,
        Self::SouthWest,
        Self::South,
        Self::SouthEast,
    ];
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coordinate(isize, isize);

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Coordinate {
    fn from_direction(&self, d: &Direction) -> Self {
        match d {
            NorthWest => Self(self.0 - 1, self.1 - 1),
            North => Self(self.0.clone(), self.1 - 1),
            NorthEast => Self(self.0 + 1, self.1 - 1),
            West => Self(self.0 - 1, self.1.clone()),
            East => Self(self.0 + 1, self.1.clone()),
            SouthWest => Self(self.0 - 1, self.1 + 1),
            South => Self(self.0.clone(), self.1 + 1),
            SouthEast => Self(self.0 + 1, self.1 + 1),
        }
    }
}

fn parse(input: &str) -> HashMap<Coordinate, char> {
    let mut map: HashMap<Coordinate, char> = HashMap::new();

    for (li, lv) in input.lines().enumerate() {
        for (ci, cv) in lv.chars().enumerate() {
            map.insert(Coordinate(li as isize, ci as isize), cv);
        }
    }

    map
}

fn caculate_xmas(coord: &Coordinate, map: &Map) -> u32 {
    let mut hits = 0;

    // println!(">> Handling {coord}");

    'direction_loop: for dir in Direction::VALUES {
        // dbg!(&dir);
        let mut current: Coordinate = coord.from_direction(&dir);
        for expected in &WANTED_CHARS {
            let Some(coord_char) = map.get(&current) else {
                continue 'direction_loop;
            };

            if !coord_char.eq(expected) {
                continue 'direction_loop;
            }

            current = current.from_direction(&dir);
        }

        hits += 1;
    }

    hits
}

fn calculate_mas(coord: &Coordinate, map: &Map) -> u32 {
    // Check NW, then SE if NW = S | M
    let nw = coord.from_direction(&Direction::NorthWest);

    let Some(char) = map.get(&nw) else {
        return 0;
    };

    let expected_oposite = match char {
        'M' => 'S',
        'S' => 'M',
        _ => return 0,
    };

    let oposite_coord = coord.from_direction(&Direction::SouthEast);
    let Some(oposite) = map.get(&oposite_coord) else {
        return 0;
    };

    if !oposite.eq(&expected_oposite) {
        return 0;
    }

    // Check NE, then SW if NE = S | M
    let ne = coord.from_direction(&Direction::NorthEast);

    let Some(char) = map.get(&ne) else {
        return 0;
    };

    let expected_oposite = match char {
        'M' => 'S',
        'S' => 'M',
        _ => return 0,
    };

    let oposite_coord = coord.from_direction(&Direction::SouthWest);
    let Some(oposite) = map.get(&oposite_coord) else {
        return 0;
    };

    if !oposite.eq(&expected_oposite) {
        return 0;
    }

    return 1;
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    //let mut total = 0;

    let result: u32 = map
        .iter()
        .filter(|(_, c)| **c == START_CHAR)
        .map(|(coord, _)| caculate_xmas(coord, &map))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);

    let result: u32 = map
        .iter()
        .filter(|(_, c)| **c == START_CHAR_2)
        .map(|(coord, _)| calculate_mas(coord, &map))
        .sum();

    Some(result)
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
        assert_eq!(result, Some(9));
    }
}
