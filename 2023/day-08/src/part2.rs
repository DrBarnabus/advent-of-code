use std::collections::BTreeMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alphanumeric1, line_ending, multispace1};
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
                alphanumeric1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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

    let starting_nodes: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();

    let results = starting_nodes
        .iter()
        .map(|node| {
            // let mut visited_nodes = vec![*node];
            let mut current_node = *node;

            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(i, instruction)| {
                    let options = map.get(current_node).expect("current node should always have options");
                    let next_node = match instruction {
                        Direction::Left => options.0,
                        Direction::Right => options.1
                    };

                    if next_node.ends_with("Z") {
                        Some(i + 1)
                    } else {
                        // visited_nodes.push(next_node);
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find a valid cycle")
        })
        .collect::<Vec<usize>>();

    least_common_multiple(&results).to_string()
}

pub fn least_common_multiple(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = least_common_multiple(&nums[1..]);
    a * b / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    greatest_common_divisor(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example3.txt"));
        assert_eq!(result, "6");
    }
}
