use anyhow::{anyhow, Error, Result};
use common::instrument;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy)]
enum BoardNum {
    Marked(u8),
    Unmarked(u8),
}

#[derive(Debug)]
struct Board {
    board: [[BoardNum; 5]; 5],
    marked_cols: HashMap<usize, u8>,
    marked_rows: HashMap<usize, u8>,
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut board = [[BoardNum::Unmarked(0); 5]; 5];

        for (y, line) in s.lines().enumerate() {
            for (x, num) in line.trim().split_whitespace().enumerate() {
                board[y][x] = BoardNum::Unmarked(num.parse()?);
            }
        }

        Ok(Self {
            board,
            marked_cols: HashMap::new(),
            marked_rows: HashMap::new(),
        })
    }
}

impl Board {
    fn draw(&mut self, drawn_num: u8) {
        for row in 0..5 {
            for col in 0..5 {
                if let BoardNum::Unmarked(num) = self.board[row][col] {
                    if num == drawn_num {
                        self.board[row][col] = BoardNum::Marked(num);
                        let row_count = self.marked_rows.entry(row).or_insert(0);
                        *row_count += 1;
                        let col_count = self.marked_cols.entry(col).or_insert(0);
                        *col_count += 1;
                    }
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        if self.marked_rows.values().any(|&count| count == 5)
            || self.marked_cols.values().any(|&count| count == 5)
        {
            return true;
        }
        false
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for row in 0..5 {
            for col in 0..5 {
                if let BoardNum::Unmarked(num) = self.board[row][col] {
                    score += num as u32;
                }
            }
        }
        score
    }
}

fn solve_part1(input: &str) -> Result<u32> {
    let mut groups = input.trim().split("\n\n");

    let draws = groups
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()?;

    let mut boards: Vec<Board> = vec![];

    for group in groups {
        boards.push(Board::from_str(group)?);
    }

    for draw in draws {
        for board in &mut boards {
            board.draw(draw);
            if board.is_win() {
                return Ok(board.score() * draw as u32);
            }
        }
    }

    Err(anyhow!("No winning board found"))
}

fn solve_part2(input: &str) -> Result<u32> {
    let mut groups = input.trim().split("\n\n");

    let draws = groups
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()?;

    let mut boards: Vec<Board> = vec![];

    for group in groups {
        boards.push(Board::from_str(group)?);
    }

    for draw in draws {
        if boards.len() == 1 {
            boards[0].draw(draw);
            if boards[0].is_win() {
                return Ok(boards[0].score() * draw as u32);
            }
        } else {
            for board in &mut boards {
                board.draw(draw);
            }
            boards.retain(|board| {
                if board.is_win() {
                    return false;
                }
                return true;
            });
        }
    }

    Err(anyhow!("No winning board found"))
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 4512);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 1924);
    }
}
