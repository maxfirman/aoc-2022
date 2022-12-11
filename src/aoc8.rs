use std::cmp::max;
use crate::utils::input;

fn part1() -> u32 {
    let wood = get_wood();
    let m = wood.len();
    let n = wood[0].len();
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res += (i == 0 || j == 0 || i == m - 1 || j == n - 1 ||
                wood[i][j] > (0..i).map(|ii| wood[ii][j]).max().unwrap() ||
                wood[i][j] > (i + 1..m).map(|ii| wood[ii][j]).max().unwrap() ||
                wood[i][j] > (0..j).map(|jj| wood[i][jj]).max().unwrap() ||
                wood[i][j] > (j + 1..n).map(|jj| wood[i][jj]).max().unwrap()) as u32;
        }
    }
    return res;
}

fn part2() -> u32 {
    let wood = get_wood();

    let m = wood.len();
    let n = wood[0].len();
    let mut res: u32 = 0;

    for i in 0..m {
        for j in 0..n {
            let mut a = 0;
            for ii in (0..i).rev() {
                a += 1;
                if wood[ii][j] >= wood[i][j] { break; }
            }
            let mut b = 0;
            for ii in i + 1..m {
                b += 1;
                if wood[ii][j] >= wood[i][j] { break; }
            }
            let mut c = 0;
            for jj in (0..j).rev() {
                c += 1;
                if wood[i][jj] >= wood[i][j] { break; }
            }
            let mut d = 0;
            for jj in j + 1..n {
                d += 1;
                if wood[i][jj] >= wood[i][j] { break; }
            }
            res = max(res, a * b * c * d)
        }
    }
    return res;
}

fn get_wood() -> Vec<Vec<u32>> {
    let wood = input(8)
        .lines()
        .map(|line| line
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect())
        .collect::<Vec<Vec<u32>>>();
    wood
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { println!("{}", part1()) }

    #[test]
    fn test_part2() { println!("{}", part2()) }
}
