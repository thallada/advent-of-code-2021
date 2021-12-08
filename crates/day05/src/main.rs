use anyhow::Result;
use common::instrument;
use std::collections::HashMap;

const INPUT: &str = include_str!("input/input.txt");

fn solve_part1(input: &str) -> Result<usize> {
    let lines = input.trim().lines();

    let mut covered_points: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines {
        let mut points = line.split(" -> ");
        let start_point = points.next().unwrap();
        let mut start_point = start_point.split(',');
        let start_x = start_point.next().unwrap().parse::<i32>()?;
        let start_y = start_point.next().unwrap().parse::<i32>()?;
        let end_point = points.next().unwrap();
        let mut end_point = end_point.split(',');
        let end_x = end_point.next().unwrap().parse::<i32>()?;
        let end_y = end_point.next().unwrap().parse::<i32>()?;

        if start_x == end_x {
            if start_y <= end_y {
                for y in start_y..=end_y {
                    let entry = covered_points.entry((start_x, y)).or_insert(0);
                    *entry += 1;
                }
            } else {
                for y in end_y..=start_y {
                    let entry = covered_points.entry((start_x, y)).or_insert(0);
                    *entry += 1;
                }
            }
        } else if start_y == end_y {
            if start_x <= end_x {
                for x in start_x..=end_x {
                    let entry = covered_points.entry((x, start_y)).or_insert(0);
                    *entry += 1;
                }
            } else {
                for x in end_x..=start_x {
                    let entry = covered_points.entry((x, start_y)).or_insert(0);
                    *entry += 1;
                }
            }
        }

        println!("{},{} -> {},{}", start_x, start_y, end_x, end_y);
        for y in 0..=9 {
            for x in 0..=9 {
                let overlap = covered_points.get(&(x, y));
                if let Some(overlap) = overlap {
                    print!("{}", overlap);
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");

    }

    for y in 0..=9 {
        for x in 0..=9 {
            let overlap = covered_points.get(&(x, y));
            if let Some(overlap) = overlap {
                print!("{}", overlap);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");


    Ok(covered_points.into_values().filter(|&overlap| overlap >= 2).count())
}

fn solve_part2(input: &str) -> Result<u64> {
    let lines = input.trim().lines();

    Ok(2)
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 5);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 0);
    }
}
