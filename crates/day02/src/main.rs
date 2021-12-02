use anyhow::Result;
use common::instrument;

const INPUT: &str = include_str!("input/input.txt");

fn solve_part1(input: &str) -> Result<i32> {
    let lines = input.trim().lines();

    let mut horizontal = 0;
    let mut depth = 0;

    for line in lines {
        let mut parts = line.split(" ");
        let command = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>()?;

        match command {
            "forward" => horizontal += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => panic!("Unknown command: {}", command),
        }
    }

    Ok(horizontal * depth)
}

fn solve_part2(input: &str) -> Result<i32> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let mut parts = line.split(" ");
        let command = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>()?;

        match command {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!("Unknown command: {}", command),
        }
    }

    Ok(horizontal * depth)
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 150);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 900);
    }
}
