advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Outcome {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Unknown,
    Ascending,
    Descending,
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

    let result: Vec<Outcome> = data
        .iter()
        .map(|set| {
            let mut direction = Direction::Unknown;
            let mut window = set.windows(2).peekable();

            direction = match window.peek() {
                Some([left, right]) => match left > right {
                    true => Direction::Descending,
                    false => Direction::Ascending,
                },
                _ => unreachable!("Window should always have 2 items"),
            };

            dbg!("==============");
            dbg!(&direction);

            while let Some([left, right]) = window.next() {
                dbg!(left, right);

                let diff = left.abs_diff(*right);

                dbg!(&diff);

                if diff < 1 || diff > 3 {
                    dbg!("Fail");
                    return Outcome::Unsafe;
                }

                match direction {
                    Direction::Descending => {
                        if left < right {
                            dbg!("Fail");
                            return Outcome::Unsafe;
                        }
                    }
                    Direction::Ascending => {
                        if right < left {
                            dbg!("Fail");
                            return Outcome::Unsafe;
                        }
                    }
                    Direction::Unknown => {
                        unreachable!("This should have been covered in the peeking")
                    }
                }

                dbg!("Pass");
            }

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
