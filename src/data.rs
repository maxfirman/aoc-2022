use std::fs;

use reqwest;

pub fn get_input(day: i32) -> String {
    let cookie = fs::read_to_string("data/cookie.txt").unwrap();
    reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("Cookie", cookie)
        .send()
        .unwrap()
        .text()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        println!("{}", get_input(2));
    }
}