use std::iter::Peekable;

advent_of_code::solution!(3);

const ORDER: [char; 6] = ['m', 'u', 'l', '(', ',', ')'];
const TOGGLE: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];

fn regex(input: &str) -> u32 {
    0
}

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

fn try_parse_mul(iter: &impl Iterator<Item = char>) -> Result<u32, &'static str> {
    Ok(0)
}

fn try_parse_toggle() -> Result<bool, &'static str> {
    //let Some(_) = iter.next_if(|c| c == &TOGGLE[1]) else {
    //    return Err("Not expected char");
    //};
    //
    //let Some(_) = char_iter.next_if(|c| c == &TOGGLE[2]) else {
    //    continue;
    //};
    //
    //dbg!(char_iter.peek());
    //match char_iter.peek() {
    //    Some(c) if c == &TOGGLE[5] => {
    //        println!("We hit left paren");
    //        char_iter.next();
    //
    //        println!("Peek: {}", char_iter.peek().unwrap());
    //        let Some(_) = char_iter.next_if(|c| c == &TOGGLE[6]) else {
    //            continue;
    //        };
    //
    //        println!("Toggle is ON!");
    //        toggle_on = true;
    //        continue;
    //    }
    //    Some(c) if c == &TOGGLE[3] => {
    //        char_iter.next();
    //    }
    //    _ => {
    //        continue;
    //    }
    //}
    //
    //let Some(_) = char_iter.next_if(|c| c == &TOGGLE[4]) else {
    //    continue;
    //};
    //
    //let Some(_) = char_iter.next_if(|c| c == &TOGGLE[5]) else {
    //    continue;
    //};
    //
    //let Some(_) = char_iter.next_if(|c| c == &TOGGLE[6]) else {
    //    continue;
    //};
    //
    //println!("Toggle is OFF");
    Ok(true)
}

fn char_walk_two(input: &str) -> u32 {
    let mut total = 0;
    let mut toggle_on = true;
    let mut index: usize = 0;
    let chars = input.ch;
    let chunk_size: usize = 20;

    loop {
        match chars[index..index + chunk_size] {
            ['d', 'o', '(', ')'] => {
                toggle_on = true;
            }
            ['d', 'o', 'n', '\'', 't', '(', ')'] => {
                toggle_on = false;
            }
            ['m', 'u', 'l', '(', rest] => {
                println!("{}", &rest);

                //while let Some(num) = rest.next_if(|c| c.is_digit(10)) {
                //    let parsed_num = num.to_digit(10).unwrap();
                //    if left == 0 {
                //        left = parsed_num;
                //        continue;
                //    }
                //
                //    left = left * 10 + parsed_num;
                //}
                //
                //let Some(_) = char_iter.next_if(|c| c == &ORDER[4]) else {
                //    continue;
                //};
                //
                //while let Some(num) = char_iter.next_if(|c| c.is_digit(10)) {
                //    let parsed_num = num.to_digit(10).unwrap();
                //    if right == 0 {
                //        right = parsed_num;
                //        continue;
                //    }
                //
                //    right = right * 10 + parsed_num;
                //}
                //
                //let Some(_) = char_iter.next_if(|c| c == &ORDER[5]) else {
                //    continue;
                //};
                //
                //total += left * right;
            }
            _ => {}
        }

        index += 1;
    }

    //for line in input.lines() {
    //    let mut char_iter = line.chars().peekable();
    //
    //    while let Some(c) = char_iter.next() {
    //        println!("Char: {}", &c);
    //        match c {
    //            c if c == TOGGLE[0] => {
    //                let Some(_) = char_iter.next_if(|c| c == &TOGGLE[1]) else {
    //                    continue;
    //                };
    //
    //                let Some(_) = char_iter.next_if(|c| c == &TOGGLE[2]) else {
    //                    continue;
    //                };
    //
    //                dbg!(char_iter.peek());
    //                match char_iter.peek() {
    //                    Some(c) if c == &TOGGLE[5] => {
    //                        println!("We hit left paren");
    //                        char_iter.next();
    //
    //                        println!("Peek: {}", char_iter.peek().unwrap());
    //                        let Some(_) = char_iter.next_if(|c| c == &TOGGLE[6]) else {
    //                            continue;
    //                        };
    //
    //                        println!("Toggle is ON!");
    //                        toggle_on = true;
    //                        continue;
    //                    }
    //                    Some(c) if c == &TOGGLE[3] => {
    //                        char_iter.next();
    //                    }
    //                    _ => {
    //                        continue;
    //                    }
    //                }
    //
    //                let Some(_) = char_iter.next_if(|c| c == &TOGGLE[4]) else {
    //                    continue;
    //                };
    //
    //                let Some(_) = char_iter.next_if(|c| c == &TOGGLE[5]) else {
    //                    continue;
    //                };
    //
    //                let Some(_) = char_iter.next_if(|c| c == &TOGGLE[6]) else {
    //                    continue;
    //                };
    //
    //                println!("Toggle is OFF");
    //                toggle_on = false;
    //            }
    //            c if c == ORDER[0] && toggle_on => {
    //                let mut left = 0;
    //                let mut right = 0;
    //
    //                if !c.eq(&ORDER[0]) {
    //                    continue;
    //                }
    //
    //                let Some(_) = char_iter.next_if(|c| c == &ORDER[1]) else {
    //                    continue;
    //                };
    //
    //                let Some(_) = char_iter.next_if(|c| c == &ORDER[2]) else {
    //                    continue;
    //                };
    //
    //                let Some(_) = char_iter.next_if(|c| c == &ORDER[3]) else {
    //                    continue;
    //                };
    //
    //                while let Some(num) = char_iter.next_if(|c| c.is_digit(10)) {
    //                    let parsed_num = num.to_digit(10).unwrap();
    //                    if left == 0 {
    //                        left = parsed_num;
    //                        continue;
    //                    }
    //
    //                    left = left * 10 + parsed_num;
    //                }
    //
    //                let Some(_) = char_iter.next_if(|c| c == &ORDER[4]) else {
    //                    continue;
    //                };
    //
    //                while let Some(num) = char_iter.next_if(|c| c.is_digit(10)) {
    //                    let parsed_num = num.to_digit(10).unwrap();
    //                    if right == 0 {
    //                        right = parsed_num;
    //                        continue;
    //                    }
    //
    //                    right = right * 10 + parsed_num;
    //                }
    //
    //                let Some(_) = char_iter.next_if(|c| c == &ORDER[5]) else {
    //                    continue;
    //                };
    //
    //                total += left * right;
    //            }
    //            _ => {
    //                continue;
    //            }
    //        }
    //    }
    //}

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
