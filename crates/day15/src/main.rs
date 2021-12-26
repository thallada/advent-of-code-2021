use anyhow::{anyhow, Result};
use common::instrument;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashMap};
use std::convert::From;

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OpenPoint {
    x: i32,
    y: i32,
    f_score: i32,
}

impl Ord for OpenPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for OpenPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Point> for OpenPoint {
    fn from(point: Point) -> Self {
        OpenPoint {
            x: point.x,
            y: point.y,
            f_score: 0,
        }
    }
}

impl From<OpenPoint> for Point {
    fn from(open_point: OpenPoint) -> Self {
        Point {
            x: open_point.x,
            y: open_point.y,
        }
    }
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: Point, start: Point) -> Vec<Point> {
    let mut path = vec![current];
    let mut current = current;
    loop {
        current = came_from[&current];
        if current == start {
            break;
        }
        path.push(current);
    }
    path
}

fn find_shortest_path(grid: &Vec<Vec<usize>>) -> Result<Vec<Point>> {
    let destination = Point {
        x: grid[0].len() as i32 - 1,
        y: grid.len() as i32 - 1,
    };
    let mut open_list = BinaryHeap::new();
    open_list.push(OpenPoint {
        x: 0,
        y: 0,
        f_score: 0,
    });
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(Point { x: 0, y: 0 }, 0);
    let mut f_score = HashMap::new();
    f_score.insert(Point { x: 0, y: 0 }, 0);

    // A* algorithm
    while !open_list.is_empty() {
        let current = open_list.pop().unwrap();
        if current.x == destination.x && current.y == destination.y {
            let path = reconstruct_path(&came_from, current.into(), Point { x: 0, y: 0 });
            return Ok(path);
        }

        let mut neighbors = vec![];
        if current.x > 0 {
            neighbors.push(Point {
                x: current.x - 1,
                y: current.y,
            });
        }
        if current.x < grid.len() as i32 - 1 {
            neighbors.push(Point {
                x: current.x + 1,
                y: current.y,
            });
        }
        if current.y > 0 {
            neighbors.push(Point {
                x: current.x,
                y: current.y - 1,
            });
        }
        if current.y < grid[0].len() as i32 - 1 {
            neighbors.push(Point {
                x: current.x,
                y: current.y + 1,
            });
        }

        for neighbor in neighbors {
            let tentative_g_score =
                g_score[&current.into()] + grid[neighbor.y as usize][neighbor.x as usize];

            let neighbor_g_score = *g_score.get(&neighbor).unwrap_or(&std::usize::MAX);
            if tentative_g_score < neighbor_g_score {
                came_from.insert(neighbor, current.into());
                g_score.insert(neighbor, tentative_g_score);
                let neighbor_f_score =
                    tentative_g_score + neighbor.manhattan_distance(&destination);
                f_score.insert(neighbor, neighbor_f_score);
                if open_list
                    .iter()
                    .find(|&p| p.x == neighbor.x && p.y == neighbor.y)
                    == None
                {
                    open_list.push(OpenPoint {
                        x: neighbor.x,
                        y: neighbor.y,
                        f_score: neighbor_f_score as i32,
                    });
                }
            }
        }
    }

    Err(anyhow!("no path found"))
}

fn solve_part1(input: &str) -> Result<i32> {
    let lines = input.trim().lines();
    let mut grid = vec![];

    for line in lines {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).ok_or(anyhow!("invalid risk level"))? as usize);
        }
        grid.push(row);
    }

    let path = find_shortest_path(&grid)?;
    Ok(path
        .iter()
        .map(|p| grid[p.y as usize][p.x as usize] as i32)
        .sum())
}

fn expand_grid(grid: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut new_grid = vec![];
    for y in 0..(grid.len() * n) {
        let mut new_row = vec![];
        for x in 0..(grid[0].len() * n) {
            let mut orig = grid[y % grid.len()][x % grid[0].len()];
            orig += x / grid[0].len() + y / grid.len();
            if orig > 9 {
                orig = (orig % 10) + 1;
            }
            new_row.push(orig);
        }
        new_grid.push(new_row);
    }

    new_grid
}

fn solve_part2(input: &str) -> Result<i32> {
    let lines = input.trim().lines();
    let mut grid = vec![];

    for line in lines {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).ok_or(anyhow!("invalid risk level"))? as usize);
        }
        grid.push(row);
    }

    let grid = expand_grid(&grid, 5);

    let path = find_shortest_path(&grid)?;
    Ok(path
        .iter()
        .map(|p| grid[p.y as usize][p.x as usize] as i32)
        .sum())
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 40);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 315);
    }
}
