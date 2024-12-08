use nom::{bytes::complete::tag, character::complete::{digit1, line_ending, newline}, combinator::map_res, error::Error, multi::separated_list1, sequence::{preceded, separated_pair, terminated}, IResult};

advent_of_code::solution!(5);

fn parse_updates(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(
    preceded(
        newline, 
        separated_list1(
            tag(","),
            map_res(digit1, |s: &str| s.parse::<u32>())
            )
        ),
    newline
    )(input)
}

fn parse_ordering(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        line_ending,
        separated_pair(
        map_res(digit1, |s: &str| s.parse::<u32>()),
        tag("|"),
        map_res(digit1, |s: &str| s.parse::<u32>()))
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let Ok((rest, orderings)) = parse_ordering(input) else {
        panic!("unable to parse orderings");
    };

    dbg!(rest);

    let Ok((rest, updates)) = parse_updates(rest) else {
        panic!("unable to parse updates");
    };

    dbg!(rest, orderings, updates);

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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
