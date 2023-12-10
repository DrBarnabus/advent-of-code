use std::ops::Not;
use itertools::Itertools;

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let start_numbers = std::iter::successors(Some(numbers), |numbers| {
                numbers
                    .iter()
                    .all(|number| number == &0)
                    .not()
                    .then_some(
                        numbers
                            .iter()
                            .tuple_windows::<(&i64, &i64)>()
                            .map(|(left, right)| right - left)
                            .collect()
                    )
            })
                .map(|v| *v.first().unwrap())
                .collect::<Vec<i64>>();

            start_numbers
                .iter()
                .rev()
                .fold(0, |accumulator, number| {
                    number - accumulator
                })
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!("2", process(input));
    }
}
