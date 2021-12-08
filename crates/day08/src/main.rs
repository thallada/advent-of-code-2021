use anyhow::Result;
use common::instrument;
use std::collections::HashSet;

const INPUT: &str = include_str!("input/input.txt");

fn solve_part1(input: &str) -> Result<i32> {
    let lines = input.trim().lines();

    let mut digit_count = 0;
    for line in lines {
        let mut parts = line.split(" | ");
        let _signal_patterns: Vec<&str> = parts.next().unwrap().trim().split(" ").collect();
        let output: Vec<&str> = parts.next().unwrap().trim().split(" ").collect();

        for digit in output {
            let mut chars = HashSet::new();
            for char in digit.chars() {
                chars.insert(char);
            }

            if chars.len() == 2 || chars.len() == 3 || chars.len() == 4 || chars.len() == 7 {
                digit_count += 1;
            }
        }
    }

    Ok(digit_count)
}

fn solve_part2(input: &str) -> Result<u32> {
    let lines = input.trim().lines();

    let mut output_nums = Vec::new();

    for line in lines {
        let mut parts = line.split(" | ");
        let mut signal_patterns: Vec<&str> = parts.next().unwrap().trim().split(" ").collect();
        let output: Vec<&str> = parts.next().unwrap().trim().split(" ").collect();

        let mut one = HashSet::new();
        let mut seven = HashSet::new();
        let mut four = HashSet::new();
        let mut eight = HashSet::new();
        signal_patterns.retain(|digit| {
            if digit.len() == 2 {
                for char in digit.chars() {
                    one.insert(char);
                }
                return false;
            } else if digit.len() == 3 {
                for char in digit.chars() {
                    seven.insert(char);
                }
                return false;
            } else if digit.len() == 4 {
                for char in digit.chars() {
                    four.insert(char);
                }
                return false;
            } else if digit.len() == 7 {
                for char in digit.chars() {
                    eight.insert(char);
                }
                return false;
            }
            return true;
        });

        let mut zero = HashSet::new();
        let mut six = HashSet::new();
        let mut nine = HashSet::new();
        signal_patterns.retain(|digit| {
            if digit.len() == 6 {
                let mut chars = HashSet::new();
                for char in digit.chars() {
                    chars.insert(char);
                }

                if chars.is_superset(&four) {
                    nine = chars;
                } else if chars.is_superset(&one) {
                    zero = chars;
                } else {
                    six = chars;
                }
                return false;
            }
            return true;
        });

        let top_right = eight.difference(&six).next().unwrap();
        let bottom_left = eight.difference(&nine).next().unwrap();

        let mut two = HashSet::new();
        let mut three = HashSet::new();
        let mut five = HashSet::new();
        for digit in signal_patterns {
            let mut chars = HashSet::new();
            for char in digit.chars() {
                chars.insert(char);
            }

            if !chars.contains(&top_right) && !chars.contains(&bottom_left) {
                five = chars;
            } else if !chars.contains(&bottom_left) {
                three = chars;
            } else {
                two = chars;
            }
        }

        output_nums.push(
            output
                .iter()
                .map(|digit| {
                    let mut chars = HashSet::new();
                    for char in digit.chars() {
                        chars.insert(char);
                    }

                    if chars == zero {
                        return '0';
                    } else if chars == one {
                        return '1';
                    } else if chars == two {
                        return '2';
                    } else if chars == three {
                        return '3';
                    } else if chars == four {
                        return '4';
                    } else if chars == five {
                        return '5';
                    } else if chars == six {
                        return '6';
                    } else if chars == seven {
                        return '7';
                    } else if chars == eight {
                        return '8';
                    } else if chars == nine {
                        return '9';
                    } else {
                        panic!("Invalid output digit");
                    }
                })
                .collect::<String>()
                .parse::<u32>()?,
        )
    }

    Ok(output_nums.into_iter().sum())
}

fn main() {
    instrument!(solve_part1(INPUT).unwrap(), solve_part2(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("input/test.txt");

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 26);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 61229);
    }
}
