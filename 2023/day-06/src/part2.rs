use nom::bytes::complete::is_not;
use nom::character::complete::{digit1, line_ending, space1};
use nom::{IResult, Parser};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom_supreme::ParserExt;

fn parse_numbers(input: &str) -> IResult<&str, u64> {
    is_not("0123456789")
        .precedes(separated_list1(space1, digit1).map(|digits| {
            digits.join("").parse::<u64>().expect("a valid number")
        }))
        .parse(input)
}

fn parse_times(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_numbers, line_ending, parse_numbers).parse(input)
}

pub fn process(input: &str) -> String {
    let (_, (time, record_distance)) = parse_times(input).expect("valid parsed data");

    (0..time)
        .filter_map(|speed| {
            let distance = (time - speed) * speed;
            (distance > record_distance).then_some(distance)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "71503");
    }
}
