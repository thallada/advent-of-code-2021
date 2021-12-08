use anyhow::Result;
use common::instrument;
use std::collections::HashMap;

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
fn print_ocean_floor(covered_points: &HashMap<Point, i32>) {
    let max_x = covered_points.keys().map(|p| p.x).max().unwrap();
    let max_y = covered_points.keys().map(|p| p.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let overlap = covered_points.get(&Point { x, y });
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

fn parse_vents(input: &str) -> Result<Vec<(Point, Point)>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ");
            let start_point = points.next().unwrap();
            let mut start_point = start_point.split(',');
            let start_x = start_point.next().unwrap().parse::<i32>()?;
            let start_y = start_point.next().unwrap().parse::<i32>()?;
            let end_point = points.next().unwrap();
            let mut end_point = end_point.split(',');
            let end_x = end_point.next().unwrap().parse::<i32>()?;
            let end_y = end_point.next().unwrap().parse::<i32>()?;
            Ok((
                Point {
                    x: start_x,
                    y: start_y,
                },
                Point { x: end_x, y: end_y },
            ))
        })
        .collect()
}

fn get_covered_points(vents: Vec<(Point, Point)>) -> Result<HashMap<Point, i32>> {
    let mut covered_points: HashMap<Point, i32> = HashMap::new();

    for (start, end) in vents.iter() {
        if start.x == end.x {
            if start.y <= end.y {
                for y in start.y..=end.y {
                    let entry = covered_points.entry(Point { x: start.x, y }).or_insert(0);
                    *entry += 1;
                }
            } else {
                for y in end.y..=start.y {
                    let entry = covered_points.entry(Point { x: start.x, y }).or_insert(0);
                    *entry += 1;
                }
            }
        } else if start.y == end.y {
            if start.x <= end.x {
                for x in start.x..=end.x {
                    let entry = covered_points.entry(Point { x, y: start.y }).or_insert(0);
                    *entry += 1;
                }
            } else {
                for x in end.x..=start.x {
                    let entry = covered_points.entry(Point { x, y: start.y }).or_insert(0);
                    *entry += 1;
                }
            }
        } else {
            let mut x = start.x;
            let mut y = start.y;
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            while (dx > 0 && x <= end.x) || (dx < 0 && x >= end.x) {
                let entry = covered_points.entry(Point { x, y }).or_insert(0);
                *entry += 1;
                if dx > 0 {
                    x += 1;
                } else {
                    x -= 1;
                }
                if dy > 0 {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    Ok(covered_points)
}

fn solve_part1(input: &str) -> Result<usize> {
    let vents = parse_vents(input)?;
    let vents = vents
        .into_iter()
        .filter(|(start, end)| start.x == end.x || start.y == end.y)
        .collect();
    let covered_points = get_covered_points(vents)?;

    Ok(covered_points
        .into_values()
        .filter(|&overlap| overlap >= 2)
        .count())
}

fn solve_part2(input: &str) -> Result<usize> {
    let vents = parse_vents(input)?;
    let covered_points = get_covered_points(vents)?;

    Ok(covered_points
        .into_values()
        .filter(|&overlap| overlap >= 2)
        .count())
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
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 12);
    }
}
