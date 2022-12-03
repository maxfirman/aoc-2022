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
        .split("\n")
        .map(|x| {
            let (a, b) = x.split_at(x.len() / 2);
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            a.intersection(&b).map(score).sum::<i32>()
        })
        .sum()
}

fn part2() -> i32 {
    get_input(3)
        .split("\n")
        .map(|x| x.chars().collect::<HashSet<_>>())
        .enumerate()
        .group_by(|(i, _)| i / 3)
        .into_iter()
        .map(|(_, x)| x
            .map(|(_, x)| x)
            .reduce(|a, b| a
                .intersection(&b)
                .map(|x| *x)
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
