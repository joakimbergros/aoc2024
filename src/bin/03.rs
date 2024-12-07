use std::{iter::Peekable, str::Chars};

advent_of_code::solution!(3);

const ORDER: [char; 6] = ['m', 'u', 'l', '(', ',', ')'];
const TOGGLE: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];

fn char_walk_one(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let mut char_iter = line.chars().peekable();

        while let Some(c) = char_iter.next() {
            let mut left = 0;
            let mut right = 0;

            if !c.eq(&ORDER[0]) {
                continue;
            }

            let Some(_) = char_iter.next_if(|c| c == &ORDER[1]) else {
                continue;
            };

            let Some(_) = char_iter.next_if(|c| c == &ORDER[2]) else {
                continue;
            };

            let Some(_) = char_iter.next_if(|c| c == &ORDER[3]) else {
                continue;
            };

            while let Some(num) = char_iter.next_if(|c| c.is_digit(10)) {
                let parsed_num = num.to_digit(10).unwrap();
                if left == 0 {
                    left = parsed_num;
                    continue;
                }

                left = left * 10 + parsed_num;
            }

            let Some(_) = char_iter.next_if(|c| c == &ORDER[4]) else {
                continue;
            };

            while let Some(num) = char_iter.next_if(|c| c.is_digit(10)) {
                let parsed_num = num.to_digit(10).unwrap();
                if right == 0 {
                    right = parsed_num;
                    continue;
                }

                right = right * 10 + parsed_num;
            }

            let Some(_) = char_iter.next_if(|c| c == &ORDER[5]) else {
                continue;
            };

            total += left * right;
        }
    }

    total
}

fn char_walk_two(input: &str) -> u32 {
    let mut total = 0;
    let mut green_light = true;

    for line in input.lines() {
        let mut char_iter = line.chars().peekable();

        while let Some(c) = char_iter.next() {
            println!("{}", &c);

            match c {
                'm' => {
                    if !green_light {
                        continue;
                    }

                    let mut left = 0;
                    let mut right = 0;

                    let Some(_) = char_iter.next_if(|c| *c == 'u') else {
                        continue;
                    };
        
                    let Some(_) = char_iter.next_if(|c| *c == 'l') else {
                        continue;
                    };
        
                    let Some(_) = char_iter.next_if(|c| *c == '(') else {
                        continue;
                    };
        
                    while let Some(num) = char_iter.next_if(|c| c.is_digit(10)) {
                        let parsed_num = num.to_digit(10).unwrap();
                        if left == 0 {
                            left = parsed_num;
                            continue;
                        }
        
                        left = left * 10 + parsed_num;
                    }
        
                    let Some(_) = char_iter.next_if(|c| *c == ',') else {
                        continue;
                    };
        
                    while let Some(num) = char_iter.next_if(|c| c.is_digit(10)) {
                        let parsed_num = num.to_digit(10).unwrap();
                        if right == 0 {
                            right = parsed_num;
                            continue;
                        }
        
                        right = right * 10 + parsed_num;
                    }
        
                    let Some(_) = char_iter.next_if(|c| *c == ')') else {
                        continue;
                    };
        
                    total += left * right;
                },
                'd' => {
                    let Some(_) = char_iter.next_if(|c| *c == 'o') else {
                        continue;
                    };

                    let Some(nc) = char_iter.next_if(|c| *c == '(' || *c == 'n') else {
                        continue;
                    };

                    match nc {
                        '(' => {
                            let Some(_) = char_iter.next_if(|c| *c == ')') else {
                                continue;
                            };

                            green_light = true;
                            continue;
                        },
                        'n' => {
                            let Some(_) = char_iter.next_if(|c| *c == '\'') else {
                                continue;
                            };

                            let Some(_) = char_iter.next_if(|c| *c == 't') else {
                                continue;
                            };

                            let Some(_) = char_iter.next_if(|c| *c == '(') else {
                                continue;
                            };

                            let Some(_) = char_iter.next_if(|c| *c == ')') else {
                                continue;
                            };

                            green_light = false;
                            continue;
                        },
                        _ => unreachable!()
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }

    total
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(char_walk_one(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(char_walk_two(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
