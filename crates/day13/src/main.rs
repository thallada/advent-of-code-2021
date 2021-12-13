use anyhow::{anyhow, Error, Result};
use common::instrument;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter, Write};
use std::str::FromStr;

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    index: i32,
}

impl FromStr for Axis {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "fold along x" => Ok(Self::X),
            "fold along y" => Ok(Self::Y),
            _ => Err(anyhow!("invalid axis: {}", s)),
        }
    }
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut fold_parts = s.split('=');
        let axis = fold_parts
            .next()
            .ok_or(anyhow!("missing axis"))?
            .trim()
            .parse()?;
        let index = fold_parts
            .next()
            .ok_or(anyhow!("missing index"))?
            .trim()
            .parse()?;
        Ok(Self { axis, index })
    }
}

#[derive(Debug)]
struct Paper {
    dots: HashMap<Point, bool>,
}

impl FromStr for Paper {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut dots = HashMap::new();

        for line in s.trim().lines() {
            let mut parts = line.split(',');
            let x = parts
                .next()
                .ok_or(anyhow!("missing x position for dot"))?
                .trim()
                .parse()?;
            let y = parts
                .next()
                .ok_or(anyhow!("missing y position for dot"))?
                .trim()
                .parse()?;
            dots.insert(Point { x, y }, true);
        }

        Ok(Self { dots })
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max = self.max();

        for y in 0..=max.y {
            for x in 0..=max.x {
                let is_dot = self.dots.get(&Point { x, y }).unwrap_or(&false);
                write!(f, "{}", if *is_dot { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Paper {
    fn max(&self) -> Point {
        let x = self.dots.keys().map(|p| p.x).max().unwrap();
        let y = self.dots.keys().map(|p| p.y).max().unwrap();

        Point { x, y }
    }

    fn fold(&mut self, fold: &Fold) {
        let mut new_dots = HashMap::new();

        for (point, is_dot) in &self.dots {
            let new_point = match fold.axis {
                Axis::X => {
                    if point.x == fold.index {
                        continue;
                    } else if point.x < fold.index {
                        *point
                    } else {
                        Point {
                            x: fold.index - (point.x - fold.index),
                            y: point.y,
                        }
                    }
                }
                Axis::Y => {
                    if point.y == fold.index {
                        continue;
                    } else if point.y < fold.index {
                        *point
                    } else {
                        Point {
                            x: point.x,
                            y: fold.index - (point.y - fold.index),
                        }
                    }
                }
            };
            new_dots.insert(new_point, *is_dot);
        }

        self.dots = new_dots;
    }
}

fn solve_part1(input: &str) -> Result<usize> {
    let mut input_parts = input.split("\n\n");
    let mut paper: Paper = input_parts
        .next()
        .ok_or(anyhow!("missing dots input"))?
        .parse()?;

    let folds = input_parts
        .next()
        .ok_or(anyhow!("missing folds input"))?
        .trim()
        .lines()
        .map(|fold| fold.parse())
        .collect::<Result<Vec<Fold>>>()?;

    paper.fold(folds.first().unwrap());

    Ok(paper.dots.values().filter(|&is_dot| *is_dot).count())
}

fn solve_part2(input: &str) -> Result<String> {
    let mut input_parts = input.split("\n\n");
    let mut paper: Paper = input_parts
        .next()
        .ok_or(anyhow!("missing dots input"))?
        .parse()?;

    let folds = input_parts
        .next()
        .ok_or(anyhow!("missing folds input"))?
        .trim()
        .lines()
        .map(|fold| fold.parse())
        .collect::<Result<Vec<Fold>>>()?;

    let mut output = String::new();

    for fold in folds {
        paper.fold(&fold);
    }

    writeln!(output)?;
    write!(output, "{}", &paper)?;

    Ok(output)
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 17);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            solve_part2(TEST_INPUT).unwrap(),
            r#"
#####
#...#
#...#
#...#
#####
"#
        );
    }
}
