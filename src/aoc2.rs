use utils::input;
use crate::utils;

enum Weapon {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

use Weapon::*;
use Outcome::*;


fn weapon_a(a: &str) -> Weapon {
    match a {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!()
    }
}

fn weapon_b(b: &str) -> Weapon {
    match b {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!()
    }
}

fn outcome_b(b: &str) -> Outcome {
    match b {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!()
    }
}

fn outcome_score(outcome: Outcome) -> i32 {
    match outcome {
        Lose => 0,
        Draw => 3,
        Win => 6
    }
}

fn weapon_score(weapon: Weapon) -> i32 {
    match weapon {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    }
}

fn total_score(outcome: Outcome, b: Weapon) -> i32 {
    outcome_score(outcome) + weapon_score(b)
}

fn part1(a: &str, b: &str) -> i32 {
    let a = weapon_a(a);
    let b = weapon_b(b);

    let outcome = match a {
        Rock => match b {
            Rock => Draw,
            Paper => Win,
            Scissors => Lose
        },
        Paper => match b {
            Rock => Lose,
            Paper => Draw,
            Scissors => Win
        },
        Scissors => match b {
            Rock => Win,
            Paper => Lose,
            Scissors => Draw
        },
    };

    return total_score(outcome, b);
}

fn part2(a: &str, b: &str) -> i32 {
    let a = weapon_a(a);
    let outcome = outcome_b(b);

    let b = match outcome {
        Lose => match a {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        },
        Draw => a,
        Win => match a {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        },
    };
    return total_score(outcome, b);
}

fn run(f: fn(&str, &str) -> i32) -> i32 {
    input(2)
        .lines()
        .map(|x| if let [a, b] = x.split_whitespace().collect::<Vec<_>>().as_slice() { (*a, *b) } else { panic!() })
        .map(|(a, b)| f(a, b))
        .sum()
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
