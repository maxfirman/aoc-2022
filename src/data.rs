use reqwest;

pub fn get_input(day: i32) -> String {
    reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("Cookie", "_ga=GA1.2.622462658.1669982214; _gid=GA1.2.1234663563.1669982214; session=53616c7465645f5fadbecffadd607bfe1f4a0a77c9f16e050a2796133220e20faebda3b585f4d6483934eb4be60df368ad78611a79a6791ad014d7727f9dc4e9")
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