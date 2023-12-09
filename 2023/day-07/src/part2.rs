use std::ops::Deref;
use itertools::{Itertools, Position};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Card::*;

        match value {
            'A' => Ok(A),
            'K' => Ok(K),
            'Q' => Ok(Q),
            'T' => Ok(T),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            'J' => Ok(J),
            _ => Err("Invalid character")
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct HandScore {
    hand_type: HandType,
    cards: [Card; 5],
}

fn score_hand(hand: &str) -> Result<HandScore, &str> {
    use HandType::*;

    let counts = hand.chars().counts();

    let values = if let Some(joker_count) = counts.get(&'J') {
        if *joker_count == 5 {
            "5".to_string()
        }  else {
            counts
                .iter()
                .filter_map(|(key, value)| (key != &'J').then_some(value))
                .sorted()
                .with_position()
                .map(|(position, value)| match position {
                    Position::Last | Position::Only => value + joker_count,
                    _ => *value
                })
                .join("")
        }
    } else {
        counts.values().sorted().join("")
    };

    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("Encountered `{}` which should never happen", value)
    };

    let cards: Vec<Card> = hand.chars().map(TryFrom::try_from).try_collect()?;
    let mut cards_arr: [Card; 5] = [Card::A; 5];
    cards_arr.iter_mut().set_from(cards);

    Ok(HandScore {
        hand_type,
        cards: cards_arr
    })
}

#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    bid: u32,
    score: HandScore,
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            let (hand, bid) = line.split_once(" ").expect("should be able to split");
            Hand {
                hand,
                bid: bid.parse::<u32>().expect("a valid parsed u32"),
                score: score_hand(hand).expect("hand to be scored successfully")
            }
        })
        .sorted_by_key(|hand| (hand.score.hand_type, hand.score.cards))
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "5905");
    }
}
