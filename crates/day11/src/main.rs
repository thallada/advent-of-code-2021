use anyhow::{anyhow, Error, Result};
use common::instrument;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

const INPUT: &str = include_str!("input/input.txt");

struct Cavern {
    grid: [[u8; 10]; 10],
}

impl FromStr for Cavern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (y, line) in s.trim().lines().enumerate() {
            for (x, char) in line.trim().chars().enumerate() {
                let energy = char.to_digit(10).ok_or(anyhow!("Invalid energy level"))? as u8;
                grid[y][x] = energy;
            }
        }
        Ok(Cavern { grid })
    }
}

impl Display for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                if self.grid[y][x] > 9 {
                    write!(f, "#")?;
                } else {
                    write!(f, "{}", self.grid[y][x])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Cavern {
    fn step(&mut self) -> Vec<(usize, usize)> {
        let mut flashed = vec![];
        for y in 0..10 {
            for x in 0..10 {
                self.grid[y][x] += 1;
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                let mut to_visit = vec![];
                if self.grid[y][x] > 9 && !flashed.contains(&(x, y)) {
                    flashed.push((x, y));
                    to_visit.append(&mut self.flash(x, y));

                    while to_visit.len() > 0 {
                        let (x, y) = to_visit.pop().unwrap();
                        if self.grid[y][x] > 9 && !flashed.contains(&(x, y)) {
                            flashed.push((x, y));
                            to_visit.append(&mut self.flash(x, y));
                        }
                    }

                    self.grid[y][x] = 0;
                    for (x, y) in &flashed {
                        self.grid[*y][*x] = 0;
                    }
                }
            }
        }
        flashed
    }

    fn flash(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut to_visit = vec![];
        for (dx, dy) in vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let (x, y) = (x as isize + dx, y as isize + dy);
            if y >= 0 && y < 10 && x >= 0 && x < 10 {
                let (x, y) = (x as usize, y as usize);
                self.grid[y][x] += 1;
                to_visit.push((x, y));
            }
        }
        to_visit
    }
}

fn solve_part1(input: &str) -> Result<usize> {
    let mut cavern = Cavern::from_str(input)?;

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += cavern.step().len();
    }

    Ok(flashes)
}

fn solve_part2(input: &str) -> Result<i32> {
    let mut cavern = Cavern::from_str(input)?;

    let mut step = 0;
    loop {
        step += 1;
        let flashed = cavern.step();
        if flashed.len() == 100 {
            break;
        }
    }

    Ok(step)
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 1656);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 195);
    }
}
