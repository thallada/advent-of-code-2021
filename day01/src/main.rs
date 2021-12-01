use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

use anyhow::Result;

const INPUT: &str = "input/input.txt";

fn solve_part1(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut increases = 0;
    let mut prev_reading: Option<i32> = None;
    for line in reader.lines() {
        let reading = line?.parse()?;
        if let Some(prev) = prev_reading {
            if reading > prev {
                increases += 1;
            }
        }
        prev_reading = Some(reading);
    }

    Ok(increases)
}

fn solve_part2(input_path: &str) -> Result<i32> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut increases = 0;
    let mut prev_sum: Option<i32> = None;
    let all_lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;
    for lines in all_lines.windows(3) {
        let sum = lines.iter().map(|s| s.parse::<i32>().unwrap()).sum();
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
    let mut now = Instant::now();
    println!("Part 1: {}", solve_part1(INPUT).unwrap());
    println!("(elapsed: {:?})", now.elapsed());
    now = Instant::now();
    println!("");
    println!("Part 2: {}", solve_part2(INPUT).unwrap());
    println!("(elapsed: {:?})", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 7);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 5);
    }
}
