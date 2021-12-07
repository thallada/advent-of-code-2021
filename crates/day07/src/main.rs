use anyhow::{anyhow, Result};
use common::instrument;
use std::num::ParseIntError;

const INPUT: &str = include_str!("input/input.txt");

fn parse_position(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn find_best_fuel(positions: &[u32], fuel_fn: fn(u32, u32) -> u32) -> Option<u32> {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut best_fuel = None;

    for center in min..=max {
        let fuel = positions.iter().map(|&p| fuel_fn(p, center)).sum::<u32>();

        if let Some(best) = best_fuel {
            if fuel < best {
                best_fuel = Some(fuel);
            }
        } else {
            best_fuel = Some(fuel);
        }
    }

    best_fuel
}

fn linear_fuel(position: u32, center: u32) -> u32 {
    (center as i32 - position as i32).abs() as u32
}

fn summation_fuel(position: u32, center: u32) -> u32 {
    let fuel = linear_fuel(position, center);
    (fuel * (fuel + 1)) / 2
}

fn solve_part1(input: &str) -> Result<u32> {
    let positions = parse_position(input)?;

    find_best_fuel(&positions, linear_fuel).ok_or_else(|| anyhow!("No best fuel found"))
}

fn solve_part2(input: &str) -> Result<u32> {
    let positions = parse_position(input)?;

    find_best_fuel(&positions, summation_fuel).ok_or_else(|| anyhow!("No best fuel found"))
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 37);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 168);
    }
}
