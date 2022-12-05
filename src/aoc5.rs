use itertools::Itertools;
use crate::data::get_input;


fn part1(crates: &mut Vec<Vec<char>>, n: usize, from: usize, to: usize) {
    for _ in 0..n {
        let val = crates[from].pop().unwrap();
        crates[to].push(val);
    }
}

fn part2(crates: &mut Vec<Vec<char>>, n: usize, from: usize, to: usize) {
    let mut tmp: Vec<char> = vec![];
    for _ in 0..n {
        let val = crates[from].pop().unwrap();
        tmp.push(val);
    }
    for _ in 0..n {
        let val = tmp.pop().unwrap();
        crates[to].push(val)
    }
}

fn run(f: fn(&mut Vec<Vec<char>>, usize, usize, usize)) -> String {
    let input = get_input(5);
    let (crates_input, instructions_input) = input.split_once("\n\n").unwrap();
    let mut crates: Vec<Vec<char>> = vec![vec![]];

    for (row, line) in crates_input.lines().rev().enumerate() {
        if row == 0 {
            crates.resize((line.len() + 2) / 4, vec![]);
        } else {
            for (i, c) in line
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c.is_alphabetic()) {
                crates[i].push(c);
            }
        }
    }

    for line in instructions_input.lines() {
        let (n, from, to) = line
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();
        f(&mut crates, n, from - 1, to - 1);
    }

    crates
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { println!("{}", run(part1)) }

    #[test]
    fn test_part2() { println!("{}", run(part2)) }
}
