use anyhow::{anyhow, Error, Result};
use common::instrument;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    End,
    Small(&'a str),
    Big(&'a str),
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(s: &'a str) -> Self {
        if s == "start" {
            Cave::Start
        } else if s == "end" {
            Cave::End
        } else {
            match s.chars().all(|c| c.is_uppercase()) {
                true => Cave::Big(s),
                false => Cave::Small(s),
            }
        }
    }
}

impl<'a> Display for Cave<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Cave::Start => write!(f, "start"),
            Cave::End => write!(f, "end"),
            Cave::Small(s) | Cave::Big(s) => write!(f, "{}", s),
        }?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CaveSystem<'a> {
    connections: HashMap<Cave<'a>, Vec<Cave<'a>>>,
}

impl<'a> TryFrom<&'a str> for CaveSystem<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let mut connections = HashMap::new();

        for line in s.trim().lines() {
            let mut parts = line.split('-');
            let origin = parts.next().ok_or(anyhow!("missing origin"))?;
            let origin: Cave = origin.into();
            let destination = parts.next().ok_or(anyhow!("missing destination"))?;
            let destination: Cave = destination.into();
            let entry = connections.entry(origin).or_insert_with(Vec::new);
            entry.push(destination);
            let entry = connections.entry(destination).or_insert_with(Vec::new);
            entry.push(origin);
        }

        Ok(CaveSystem { connections })
    }
}

impl<'a> CaveSystem<'a> {
    fn get_paths(&self, path: &Vec<Cave<'a>>, one_small_twice: bool) -> Result<Vec<Vec<Cave>>> {
        let mut paths = vec![];
        let origin = path.last().ok_or(anyhow!("empty path"))?;

        if let Some(destinations) = self.connections.get(&origin) {
            for destination in destinations {
                paths.append(&mut match destination {
                    Cave::Start => continue,
                    Cave::End => {
                        let mut path = path.clone();
                        path.push(*destination);
                        Ok(vec![path])
                    }
                    Cave::Big(_) => {
                        let mut path = path.clone();
                        path.push(*destination);
                        self.get_paths(&path, one_small_twice)
                    }
                    Cave::Small(_) => {
                        if !path.contains(destination) {
                            let mut path = path.clone();
                            path.push(*destination);
                            self.get_paths(&path, one_small_twice)
                        } else if one_small_twice {
                            let mut path = path.clone();
                            path.push(*destination);
                            self.get_paths(&path, false)
                        } else {
                            continue;
                        }
                    }
                }?);
            }
        }
        Ok(paths)
    }
}

fn solve_part1(input: &str) -> Result<usize> {
    let cave_system: CaveSystem = input.try_into()?;

    let paths = cave_system.get_paths(&vec![Cave::Start], false)?;

    Ok(paths.len())
}

fn solve_part2(input: &str) -> Result<usize> {
    let cave_system: CaveSystem = input.try_into()?;

    let paths = cave_system.get_paths(&vec![Cave::Start], true)?;

    Ok(paths.len())
}

fn main() {
    instrument!(solve_part1(INPUT).unwrap(), solve_part2(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");
    const TEST_INPUT2: &str = include_str!("input/test2.txt");
    const TEST_INPUT3: &str = include_str!("input/test3.txt");

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 10);
        assert_eq!(solve_part1(TEST_INPUT2).unwrap(), 19);
        assert_eq!(solve_part1(TEST_INPUT3).unwrap(), 226);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT1).unwrap(), 36);
        assert_eq!(solve_part2(TEST_INPUT2).unwrap(), 103);
        assert_eq!(solve_part2(TEST_INPUT3).unwrap(), 3509);
    }
}
