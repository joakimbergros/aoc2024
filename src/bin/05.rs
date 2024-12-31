use std::{collections::HashMap, result, usize};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(5);

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<u32>())),
    )(input)
}

fn parse_ordering(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    terminated(
        separated_list1(
            line_ending,
            separated_pair(
                map_res(digit1, |s: &str| s.parse::<u32>()),
                tag("|"),
                map_res(digit1, |s: &str| s.parse::<u32>()),
            ),
        ),
        many1(line_ending),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let Ok((rest, mut orderings)) = parse_ordering(input) else {
        panic!("unable to parse orderings");
    };

    // dbg!(&rest);

    let Ok((rest, updates)) = parse_updates(rest) else {
        panic!("unable to parse updates");
    };

    // dbg!(rest, orderings, updates);

    let rules =
        orderings
            .into_iter()
            .fold(HashMap::new(), |mut acc: HashMap<u32, Vec<u32>>, (l, r)| {
                acc.entry(l)
                    .and_modify(|v| v.push(r))
                    .or_insert_with(|| vec![r]);

                acc
            });

    let count: usize = updates
        .into_iter()
        .flat_map(|row| {
            row.iter().enumerate().map(|(i, col)| {
                rules
                    .get(col)
                    .and_then(|rs| Some(rs.iter().all(|r| row[..i].contains(col))))
            })
        })
        .filter(|x| x.is_some())
        .count();

    Some(count as u32)
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
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
