use crate::utils::input;

fn run(n: usize) -> usize {
    input(6)
        .chars()
        .collect::<Vec<_>>()
        .windows(n)
        .map(|x| (1..x.len()).any(|i| x[i..].contains(&x[i - 1])))
        .take_while(|x| *x)
        .enumerate()
        .map(|(i, _)| i + n + 1)
        .last()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { println!("{}", run(4)) }

    #[test]
    fn test_part2() { println!("{}", run(14)) }
}
