use anyhow::Result;
use common::instrument;

const INPUT: &str = include_str!("input/input.txt");

fn simulate_day(fish: &mut Vec<u8>) {
    for i in 0..fish.len() {
        if fish[i] == 0 {
            fish[i] = 6;
            fish.push(8)
        } else {
            fish[i] -= 1;
        }
    }
}

fn simulate_day_with_map(fish_map: &mut [u64; 9]) {
    fish_map.rotate_left(1);
    fish_map[6] += fish_map[8];
}

fn solve_part1(input: &str) -> Result<usize> {
    let mut fish = input
        .trim()
        .split(',')
        .map(|num| num.parse::<u8>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()?;

    for _ in 0..80 {
        simulate_day(&mut fish);
    }

    Ok(fish.len())
}

fn solve_part2(input: &str) -> Result<u64> {
    let fish = input
        .trim()
        .split(',')
        .map(|num| num.parse::<u8>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()?;

    let mut fish_map = [0; 9];

    for f in fish.into_iter() {
        fish_map[f as usize] += 1;
    }

    for _ in 0..256 {
        simulate_day_with_map(&mut fish_map);
    }

    Ok(fish_map.iter().sum())
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 5934);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 26984457539);
    }
}
