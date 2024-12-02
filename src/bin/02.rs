advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Outcome {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
enum Direction {
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

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);

    let result: Vec<Outcome> = data
        .iter()
        .map(|set| {
            let mut previous_location = None;
            let mut extra_life_used = false;

            dbg!("Handling set:");
            dbg!(set);

            let direction = match set.windows(2).next() {
                Some([left, right]) => match left > right {
                    true => Direction::Descending,
                    false => Direction::Ascending,
                },
                _ => unreachable!("Window should always have 2 items"),
            };

            let mut locations = set.iter();
            while let Some(location) = locations.next() {
                dbg!("Current: ");
                dbg!(location);
                dbg!("Previous: ");
                dbg!(previous_location);

                let Some(prev) = previous_location else {
                    previous_location = Some(location);

                    continue;
                };

                let diff = prev.abs_diff(*location);

                let out_of_bounds = !(1..=3).contains(&diff);
                let limit_exceeded = (direction == Direction::Descending && prev < location)
                    || (direction == Direction::Ascending && prev > location);

                if out_of_bounds || limit_exceeded && !extra_life_used {
                    if extra_life_used {
                        dbg!("Fail!");

                        return Outcome::Unsafe;
                    }

                    dbg!("Used extra life");

                    extra_life_used = true;

                    continue;
                }

                previous_location = Some(location);
            }

            dbg!("Pass!");

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
