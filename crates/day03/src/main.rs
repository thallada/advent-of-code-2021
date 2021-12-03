use anyhow::{anyhow, Result};
use common::instrument;

const INPUT: &str = include_str!("input/input.txt");

fn get_most_common(lines: &[&str]) -> Vec<usize> {
    let mut most_common = vec![0; lines[0].len()];

    for line in lines.iter() {
        let chars = line.chars();
        for (i, c) in chars.enumerate() {
            if c == '1' {
                most_common[i] += 1;
            }
        }
    }

    return most_common;
}

fn solve_part1(input: &str) -> Result<i32> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let most_common = get_most_common(&lines);

    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, c) in most_common.into_iter().enumerate() {
        if i > 0 {
            gamma <<= 1;
            epsilon <<= 1;
        }
        if c > (lines.len() / 2) {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    Ok(gamma * epsilon)
}

fn solve_part2(input: &str) -> Result<i32> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();

    for i in 0..lines[0].len() {
        let oxygen_most_common = get_most_common(&oxygen);
        let co2_most_common = get_most_common(&co2);

        if oxygen.len() > 1 {
            if oxygen_most_common[i] as f32 >= (oxygen.len() as f32 / 2.0) {
                oxygen = oxygen
                    .into_iter()
                    .filter(|l| l.chars().nth(i).unwrap() == '1')
                    .collect();
            } else {
                oxygen = oxygen
                    .into_iter()
                    .filter(|l| l.chars().nth(i).unwrap() == '0')
                    .collect();
            }
        }

        if co2.len() > 1 {
            if co2_most_common[i] as f32 >= (co2.len() as f32 / 2.0) {
                co2 = co2
                    .into_iter()
                    .filter(|l| l.chars().nth(i).unwrap() == '0')
                    .collect();
            } else {
                co2 = co2
                    .into_iter()
                    .filter(|l| l.chars().nth(i).unwrap() == '1')
                    .collect();
            }
        }

        if oxygen.len() == 1 && co2.len() == 1 {
            return Ok(i32::from_str_radix(oxygen[0], 2)? * i32::from_str_radix(co2[0], 2)?);
        }
    }

    Err(anyhow!("No ratings found"))
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 198);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 230);
    }
}
