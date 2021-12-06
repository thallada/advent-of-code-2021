use anyhow::Result;
use common::instrument;
use std::collections::HashMap;

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

fn simulate_day_with_map(fish_map: &HashMap<u8, u64>) -> HashMap<u8, u64> {
    let mut new_map = HashMap::new();
    for (day, count) in fish_map.into_iter() {
        if *day == 0 {
            let day8 = new_map.entry(8).or_insert(0);
            *day8 += count;
            let day6 = new_map.entry(6).or_insert(0);
            *day6 += count;
        } else {
            let day_minus = new_map.entry(day - 1).or_insert(0);
            *day_minus += count;
        }
    }
    new_map
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

    let mut fish_map: HashMap<u8, u64> = HashMap::new();

    for f in fish.iter() {
        let count = fish_map.entry(*f).or_insert(0);
        *count += 1;
    }

    for _ in 0..256 {
        fish_map = simulate_day_with_map(&fish_map);
    }

    Ok(fish_map.values().sum())
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
