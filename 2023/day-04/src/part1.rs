#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

pub fn process(input: &str) -> String {
    let cards: Vec<Card> = input
        .lines()
        .map(|line| {
            let split_by_colon: Vec<&str> = line.split(':').collect();
            let id = split_by_colon[0].replace("Card", "").trim().parse::<u32>().expect("To be the Id of the Card");

            let split_by_pipe: Vec<&str> = split_by_colon[1].trim().split('|').map(|s| s.trim()).collect();
            let winning_numbers = extract_numbers(split_by_pipe[0]);
            let numbers = extract_numbers(split_by_pipe[1]);

            Card { id, winning_numbers, numbers }
        })
        .collect();

    let mut total = 0;
    for card in cards {
        let matching_numbers = card.numbers
            .iter()
            .filter(|n| card.winning_numbers.contains(n))
            .count();

        if matching_numbers != 0 {
            total += u32::pow(2, matching_numbers as u32 - 1);
        }
    }

    total.to_string()
}

fn extract_numbers(split_line: &str) -> Vec<u32> {
    split_line
        .split(' ')
        .filter_map(|split| split.parse::<u32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "13");
    }
}
