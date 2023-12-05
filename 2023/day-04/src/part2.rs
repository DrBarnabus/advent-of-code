use std::collections::{BTreeMap, HashSet};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::{line_ending, space0, space1};
use nom::{IResult, Parser};
use nom::combinator::eof;
use nom::multi::fold_many1;
use nom::sequence::terminated;
use nom_supreme::multi::collect_separated_terminated;
use nom_supreme::ParserExt;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>
}

impl Card {
    fn count_matches(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

fn parse_set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc, item| {
            acc.insert(item);
            acc
        }
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    take_until(":")
        .precedes(tag(":").precedes(space1))
        .precedes(
            parse_set.separated_array(tag("|").precedes(space1))
                .map(|[winning_numbers, numbers]| {
                    Card { winning_numbers, numbers }
                })
        )
        .parse(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    collect_separated_terminated(parse_card, line_ending, eof).parse(input)
}

pub fn process(input: &str) -> String {
    let (_, cards) = parse_cards(input).expect("valid parsed data");

    let matched_cards = (0..cards.len())
        .map(|i| (i, 1))
        .collect::<BTreeMap<usize, u32>>();

    cards
        .iter()
        .map(|c| c.count_matches())
        .enumerate()
        .fold(matched_cards, |mut acc, (index, matches)| {
            let to_add = *acc.get(&index).unwrap();

            for i in (index + 1)..(index + 1 + matches) {
                acc.entry(i).and_modify(|v| *v += to_add);
            }

            acc
        })
        .values()
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "30");
    }
}
