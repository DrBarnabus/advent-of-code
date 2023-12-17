pub fn process(input: &str) -> String {
    input.split(',')
        .map(|hash| {
            hash.chars().fold(0, |accumulator, next_char| {
                (accumulator + (next_char as usize)) * 17 % 256
            })
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "1320");
    }
}