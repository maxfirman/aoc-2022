use std::collections::HashSet;
use itertools::Itertools;
use data::get_input;
use crate::data;

fn score(c: &char) -> i32 {
    if c.is_ascii_lowercase() {
        *c as i32 - 96
    } else {
        *c as i32 - 38
    }
}


fn part1() -> i32 {
    get_input(3)
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left = left.chars().collect::<HashSet<_>>();
            let right = right.chars().collect::<HashSet<_>>();
            left.intersection(&right).map(score).sum::<i32>()
        })
        .sum()
}

fn part2() -> i32 {
    get_input(3)
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .enumerate()
        .group_by(|(i, _)| i / 3)
        .into_iter()
        .map(|(_, group)| group
            .map(|(_, chars)| chars)
            .reduce(|left, right| left
                .intersection(&right)
                .map(|c| *c)
                .collect())
            .unwrap()
            .iter()
            .map(score)
            .sum::<i32>())
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", part1())
    }

    #[test]
    fn test_part2() {
        println!("{}", part2());
    }
}
