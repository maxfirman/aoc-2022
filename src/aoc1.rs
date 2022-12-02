use std::collections::BinaryHeap;

use data::get_input;

use crate::data;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=i32> + 'a {
    input
        .trim()
        .split("\n\n")
        .map(|x| x
            .split("\n")
            .map(|x| x.parse::<i32>().unwrap())
            .sum()
        )
}

fn part1() -> i32 {
    parse(&get_input(1))
        .max()
        .unwrap()
}

fn part2() -> i32 {
    parse(&get_input(1))
        .collect::<BinaryHeap<_>>()
        .iter()
        .take(3)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", part1());
    }

    #[test]
    fn test_part2() {
        println!("{}", part2());
    }
}