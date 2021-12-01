use anyhow::Result;
use common::instrument;

const INPUT: &str = include_str!("input/input.txt");

fn solve_part1(input: &str) -> Result<i32> {
    let lines = input.trim().lines();

    let mut increases = 0;
    let mut prev_reading: Option<i32> = None;
    for line in lines {
        let reading = line.parse()?;
        if let Some(prev) = prev_reading {
            if reading > prev {
                increases += 1;
            }
        }
        prev_reading = Some(reading);
    }

    Ok(increases)
}

fn solve_part2(input: &str) -> Result<i32> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut increases = 0;
    let mut prev_sum: Option<i32> = None;
    for group in lines.windows(3) {
        let sum = group.iter().map(|s| s.parse::<i32>().unwrap()).sum();
        if let Some(prev) = prev_sum {
            if sum > prev {
                increases += 1;
            }
        }
        prev_sum = Some(sum);
    }

    Ok(increases)
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 7);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 5);
    }
}
