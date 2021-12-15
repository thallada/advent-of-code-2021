use anyhow::{anyhow, Result};
use common::instrument;
use std::collections::HashMap;

const INPUT: &str = include_str!("input/input.txt");

fn solve_part1(input: &str) -> Result<usize> {
    let mut parts = input.split("\n\n");
    let template = parts.next().ok_or(anyhow!("no template"))?;

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for rule in parts
        .next()
        .ok_or(anyhow!("no pair insertion rules"))?
        .lines()
    {
        let mut rule_parts = rule.split(" -> ");
        let pair: Vec<char> = rule_parts
            .next()
            .ok_or(anyhow!("no pair part of rule"))?
            .chars()
            .take(2)
            .collect();
        let insertion = rule_parts
            .next()
            .ok_or(anyhow!("no pair part of rule"))?
            .chars()
            .next()
            .ok_or(anyhow!("empty insertion rule"))?;
        rules.insert((pair[0], pair[1]), insertion);
    }

    let mut counts = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut polymer = template.chars().collect::<Vec<char>>();
    for _ in 0..10 {
        let mut insertions = vec![];
        for (i, pair) in polymer.windows(2).enumerate() {
            if let Some(insertion) = rules.get(&(pair[0], pair[1])) {
                insertions.push((i + 1, *insertion));
                *counts.entry(*insertion).or_insert(0) += 1;
            }
        }
        for (i, (insertion_index, char)) in insertions.into_iter().enumerate() {
            polymer.insert(insertion_index + i, char);
        }
    }

    let max: (&char, &usize) = counts
        .iter()
        .max_by_key(|(_, count)| **count)
        .ok_or(anyhow!("no max"))?;
    let min: (&char, &usize) = counts
        .iter()
        .min_by_key(|(_, count)| **count)
        .ok_or(anyhow!("no min"))?;

    Ok(max.1 - min.1)
}

fn solve_part2(input: &str) -> Result<usize> {
    let mut parts = input.split("\n\n");
    let template = parts.next().ok_or(anyhow!("no template"))?;

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for rule in parts
        .next()
        .ok_or(anyhow!("no pair insertion rules"))?
        .lines()
    {
        let mut rule_parts = rule.split(" -> ");
        let pair: Vec<char> = rule_parts
            .next()
            .ok_or(anyhow!("no pair part of rule"))?
            .chars()
            .take(2)
            .collect();
        let insertion = rule_parts
            .next()
            .ok_or(anyhow!("no pair part of rule"))?
            .chars()
            .next()
            .ok_or(anyhow!("empty insertion rule"))?;
        rules.insert((pair[0], pair[1]), insertion);
    }

    let polymer = template.chars().collect::<Vec<char>>();
    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in polymer.windows(2) {
        *pair_counts.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_pair_counts = HashMap::new();
        for (pair, count) in pair_counts.into_iter() {
            if let Some(insertion) = rules.get(&pair) {
                *new_pair_counts.entry((pair.0, *insertion)).or_insert(0) += count;
                *new_pair_counts.entry((*insertion, pair.1)).or_insert(0) += count;
            }
        }
        pair_counts = new_pair_counts;
    }

    let mut counts = HashMap::new();
    for ((first, _), count) in pair_counts.iter() {
        *counts.entry(*first).or_insert(0) += count;
    }
    *counts
        .entry(*polymer.last().ok_or(anyhow!("empty polymer"))?)
        .or_insert(0) += 1;

    let max: (&char, &usize) = counts
        .iter()
        .max_by_key(|(_, count)| **count)
        .ok_or(anyhow!("no max"))?;
    let min: (&char, &usize) = counts
        .iter()
        .min_by_key(|(_, count)| **count)
        .ok_or(anyhow!("no min"))?;

    Ok(max.1 - min.1)
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
        assert_eq!(solve_part1(TEST_INPUT).unwrap(), 1588);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(TEST_INPUT).unwrap(), 2188189693529);
    }
}
