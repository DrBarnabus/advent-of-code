use std::ops::Range;
use nom::bytes::complete::take_until;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::{IResult, Parser};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self.mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        destination_range.start + (source - source_range.start)
    }
}

fn parse_line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, length)) = tuple((
        complete::u64, complete::u64.preceded_by(tag(" ")), complete::u64.preceded_by(tag(" "))
    ))(input)?;

    Ok((input, (source..(source + length), destination..(destination + length))))
}

fn parse_seedmap(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(
            many1(line_ending.precedes(parse_line))
                .map(|mappings| SeedMap { mappings })
        )
        .parse(input)
}

fn parse_seedmaps(input: &str) -> IResult<&str, (Vec<u64>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;

    let (input, maps) = many1(parse_seedmap)(input)?;

    Ok((input, (seeds, maps)))
}

pub fn process(input: &str) -> String {
    let (_, (seeds, maps)) = parse_seedmaps(input).expect("valid parsed data");

    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
        .min()
        .expect("should have a minimum location value")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "35");
    }
}
