advent_of_code::solution!(1);

fn split_to_tuple(line: &str) -> (u32, u32) {
    let mut ids = line.split_whitespace();

    (
        ids.next()
            .expect("Data should have a left side")
            .parse()
            .expect("Data should be numeric"),
        ids.next()
            .expect("Data should have a right side")
            .parse()
            .expect("Data should be numeric"),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut right, mut left): (Vec<u32>, Vec<u32>) =
        input.lines().fold((vec![], vec![]), |mut acc, line| {
            let ids = split_to_tuple(line);

            acc.0.push(ids.0);
            acc.1.push(ids.1);

            acc
        });

    right.sort();
    left.sort();

    let result = right
        .iter()
        .zip(&left)
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input.lines().fold((vec![], vec![]), |mut acc, line| {
        let ids = split_to_tuple(line);

        acc.0.push(ids.0);
        acc.1.push(ids.1);

        acc
    });

    let sum: u32 = data
        .0
        .iter()
        .map(|v| v * data.1.iter().filter(|n| *n == v).count() as u32)
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
