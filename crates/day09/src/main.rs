use anyhow::{anyhow, Result};
use common::instrument;

const INPUT: &str = include_str!("input/input.txt");

fn parse_grid(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).ok_or(anyhow!("Invalid height")))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

struct LowPoint {
    x: usize,
    y: usize,
    height: u32,
}

fn get_low_points(grid: &[Vec<u32>]) -> Vec<LowPoint> {
    let mut low_points = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let current = grid[y][x];
            if (y == 0 || grid[y - 1][x] > current)
                && (y == grid.len() - 1 || grid[y + 1][x] > current)
                && (x == 0 || grid[y][x - 1] > current)
                && (x == grid[y].len() - 1 || grid[y][x + 1] > current)
            {
                low_points.push(LowPoint {
                    x,
                    y,
                    height: grid[y][x],
                });
            }
        }
    }
    low_points
}

fn get_basin_size(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut to_visit = vec![(x, y)];
    let mut visited = vec![];

    while to_visit.len() > 0 {
        let (x, y) = to_visit.pop().unwrap();

        if visited.contains(&(x, y)) || grid[y][x] == 9 {
            continue;
        }
        if x > 0 {
            to_visit.push((x - 1, y));
        }
        if x < grid[y].len() - 1 {
            to_visit.push((x + 1, y));
        }
        if y > 0 {
            to_visit.push((x, y - 1));
        }
        if y < grid.len() - 1 {
            to_visit.push((x, y + 1));
        }

        visited.push((x, y));
    }

    visited.len() as u32
}

fn solve_part1(input: &str) -> Result<u32> {
    let grid = parse_grid(input)?;

    let low_points = get_low_points(&grid);

    Ok(low_points.into_iter().map(|p| p.height + 1).sum())
}

fn solve_part2(input: &str) -> Result<u32> {
    let grid = parse_grid(input)?;

    let mut basin_sizes = vec![];
    for low_point in get_low_points(&grid) {
        basin_sizes.push(get_basin_size(&grid, low_point.x, low_point.y));
    }

    basin_sizes.sort_unstable();
    Ok(basin_sizes.into_iter().rev().take(3).product())
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 15);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 1134);
    }
}
