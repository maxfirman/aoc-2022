use crate::utils::input;

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

    let outcome = match (a, &b) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Win,
        (Rock, Scissors) => Lose,
        (Paper, Rock) => Lose,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Lose,
        (Scissors, Scissors) => Draw,
    };

    return total_score(outcome, b);
}

fn part2(a: &str, b: &str) -> i32 {
    let a = weapon_a(a);
    let outcome = outcome_b(b);

    let b = match (&outcome, a) {
        (Lose, Rock) => Scissors,
        (Lose, Paper) => Rock,
        (Lose, Scissors) => Paper,
        (Draw, Rock) => Rock,
        (Draw, Paper) => Paper,
        (Draw, Scissors) => Scissors,
        (Win, Rock) => Paper,
        (Win, Paper) => Scissors,
        (Win, Scissors) => Rock
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
