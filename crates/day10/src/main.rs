use anyhow::Result;
use common::instrument;

const INPUT: &str = include_str!("input/input.txt");

fn solve_part1(input: &str) -> Result<u32> {
    let lines = input.trim().lines();

    let mut points = 0;
    for line in lines {
        let mut opens = vec![];

        for char in line.trim().chars() {
            match char {
                '(' | '[' | '{' | '<' => opens.push(char),
                ')' => {
                    if let Some(open) = opens.pop() {
                        if open != '(' {
                            points += 3;
                            break;
                        }
                    }
                }
                ']' => {
                    if let Some(open) = opens.pop() {
                        if open != '[' {
                            points += 57;
                            break;
                        }
                    }
                }
                '}' => {
                    if let Some(open) = opens.pop() {
                        if open != '{' {
                            points += 1197;
                            break;
                        }
                    }
                }
                '>' => {
                    if let Some(open) = opens.pop() {
                        if open != '<' {
                            points += 25137;
                            break;
                        }
                    }
                }
                _ => panic!("unexpected character"),
            }
        }
    }

    Ok(points)
}

fn solve_part2(input: &str) -> Result<u64> {
    let lines = input.trim().lines();

    let mut scores = vec![];
    'lines: for line in lines {
        let mut opens = vec![];
        let mut points: u64 = 0;

        for char in line.trim().chars() {
            match char {
                '(' | '[' | '{' | '<' => opens.push(char),
                ')' => {
                    if let Some(open) = opens.pop() {
                        if open != '(' {
                            continue 'lines;
                        }
                    }
                }
                ']' => {
                    if let Some(open) = opens.pop() {
                        if open != '[' {
                            continue 'lines;
                        }
                    }
                }
                '}' => {
                    if let Some(open) = opens.pop() {
                        if open != '{' {
                            continue 'lines;
                        }
                    }
                }
                '>' => {
                    if let Some(open) = opens.pop() {
                        if open != '<' {
                            continue 'lines;
                        }
                    }
                }
                _ => panic!("unexpected character"),
            }
        }

        if !opens.is_empty() {
            for open in opens.iter().rev() {
                match open {
                    '(' => points = (points * 5) + 1,
                    '[' => points = (points * 5) + 2,
                    '{' => points = (points * 5) + 3,
                    '<' => points = (points * 5) + 4,
                    _ => panic!("unexpected open character"),
                }
            }
            scores.push(points);
        }
    }

    scores.sort();
    Ok(scores[scores.len() / 2])
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 26397);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 288957);
    }
}
