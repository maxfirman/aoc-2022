use crate::utils::input;

fn directories() -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut dirs: Vec<i32> = Vec::new();

    for line in input(7).lines() {
        if line.starts_with("$ cd ..") {
            let dir_size = stack.pop().unwrap();
            let n = stack.len();
            stack[n - 1] += dir_size;
            dirs.push(dir_size);
        } else if line.starts_with("$ cd") {
            stack.push(0);
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            let size = line.split_once(" ").unwrap().0.parse::<i32>().unwrap();
            let n = stack.len();
            stack[n - 1] += size;
        }
    }
    loop {
        let dir_size = stack.pop().unwrap();
        dirs.push(dir_size);
        let n = stack.len();
        if n == 0 {
            break;
        }
        stack[n - 1] += dir_size;
    }
    dirs
}

fn part1() -> i32 {
    directories().into_iter().filter(|x| *x <= 100000).sum()
}

fn part2() -> i32 {
    let total_disk_space = 70000000;
    let desired_free_space = 30000000;
    let mut dirs = directories();
    dirs.sort();
    let used_space = dirs.last().unwrap();
    let free_space = total_disk_space - used_space;
    let minimum_directory_size_to_delete = desired_free_space - free_space;
    for x in dirs {
        if x >= minimum_directory_size_to_delete {
            return x;
        }
    }
    panic!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { println!("{}", part1()) }

    #[test]
    fn test_part2() { println!("{}", part2()) }
}
