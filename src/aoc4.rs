use itertools::Itertools;
use crate::utils::input;


fn part1(l1: i32, h1: i32, l2: i32, h2: i32) -> bool {
    (l1 <= l2 && h1 >= h2) || (l2 <= l1 && h2 >= h1)
}

fn part2(l1: i32, h1: i32, l2: i32, h2: i32) -> bool {
    (l1 <= l2 && h1 >= l2) || (l2 <= l1 && h2 >= l1)
}

fn run(f: fn(i32, i32, i32, i32) -> bool) -> i32 {
    input(4)
        .lines()
        .map(|line| line
            .split(",")
            .flat_map(|x| x.split("-"))
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple::<(i32, i32, i32, i32)>()
            .unwrap()
        )
        .map(|(l1, h1, l2, h2)| f(l1, h1, l2, h2) as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", run(part1))
    }

    #[test]
    fn test_part2() {
        println!("{}", run(part2));
    }
}
