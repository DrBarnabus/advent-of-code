use std::collections::BTreeMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, multispace1};
use nom::combinator::eof;
use nom::IResult;
use nom::multi::{fold_many1, many1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::Parser;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parser(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right)
    )))(input)?;

    let (input, _) = multispace1(input)?;

    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
                    complete::char(')')
                )
            ),
            alt((line_ending, eof))
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        }
    )(input)?;

    Ok((input, (instructions, map)))
}

pub fn process(input: &str) -> String {
    let (_, (instructions, map)) = parser(input).expect("valid parsed data");

    let mut current_node = "AAA"; // Starting at node `AAA`
    let Some(step_count) = instructions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(i, instruction)| {
            let options = map.get(current_node).expect("current node should always have options");
            let next_node = match instruction {
                Direction::Left => options.0,
                Direction::Right => options.1
            };

            if next_node == "ZZZ" {
                Some(i + 1)
            } else {
                current_node = next_node;
                None
            }
        })
    else {
        panic!("an infinate operator shouldn't produce None")
    };

    step_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = process(include_str!("example1.txt"));
        assert_eq!(result, "2");
    }

    #[test]
    fn it_works2() {
        let result = process(include_str!("example2.txt"));
        assert_eq!(result, "6");
    }
}
