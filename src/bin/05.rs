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

fn group_rules(rules: Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    rules
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<u32, Vec<u32>>, (l, r)| {
            acc.entry(l)
                .and_modify(|v| v.push(r))
                .or_insert_with(|| vec![r]);

            acc
        })
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

    let rules = group_rules(orderings);

    let mut ok_rows: Vec<Vec<u32>> = Vec::new();
    'rows: for row in updates {
        for (i, instruction) in row.iter().enumerate() {
            let Some(rule) = rules.get(instruction) else {
                continue;
            };

            if rule.iter().any(|x| row[..i].contains(x)) {
                continue 'rows;
            }
        }

        ok_rows.push(row);
    }

    let mut sum = 0;
    for row in ok_rows {
        let row_len = row.len();
        if row_len % 2 == 1 {
            sum += row[row_len / 2];
        }
    }

    dbg!(&sum);

    Some(sum)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
