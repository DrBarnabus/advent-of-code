use std::ops::RangeInclusive;

use miette::miette;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (ranges, ids)) = parse(input).map_err(|e| miette!("failed to parse input, {e}"))?;

    let result = ids
        .iter()
        .filter(|ingredient_id| ranges.iter().any(|range| range.contains(&ingredient_id)))
        .count();

    Ok(result.to_string())
}

pub fn parse(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    terminated(
        separated_pair(
            separated_list1(
                line_ending,
                separated_pair(complete::u64, tag("-"), complete::u64).map(|(a, b)| a..=b),
            ),
            line_ending.and(line_ending),
            separated_list1(line_ending, complete::u64),
        ),
        opt(line_ending),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!("3", process(input)?);
        Ok(())
    }
}
