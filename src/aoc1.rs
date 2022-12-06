use std::collections::BinaryHeap;
use std::iter::Map;
use std::str::Split;
use crate::utils::input;

type I<'a> = Map<Split<'a, &'a str>, fn(&str) -> i32>;

fn part1(iter: I) -> i32
{
    iter.max().unwrap()
}

fn part2(iter: I) -> i32
{
    iter.collect::<BinaryHeap<_>>()
        .iter()
        .take(3)
        .sum()
}

fn run(f: fn(I) -> i32) -> i32
{
    f(
        input(1)
            .split("\n\n")
            .map(|x| x
                .lines()
                .map(|x| x.parse::<i32>().unwrap())
                .sum()
            )
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", run(part1));
    }

    #[test]
    fn test_part2() {
        println!("{}", run(part2));
    }
}