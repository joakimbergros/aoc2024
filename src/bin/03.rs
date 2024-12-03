advent_of_code::solution!(3);

const ORDER: [char; 8] = ['m', 'u', 'l', '(', 'x', ',', 'x', ')'];

fn regex(input: &str) -> u32 {
    0
}

fn char_walk(input: &str) -> u32 {
    for line in input.lines() {
        let mut char_iter = line.chars();
        let mut match_index: Option<u8> = None;

        while let Some(c) = char_iter.next() {
            if match_index.is_none() && c.eq('m') {

            }

            if match_index.is_some() && ORDER.contains(x)
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    input.
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
