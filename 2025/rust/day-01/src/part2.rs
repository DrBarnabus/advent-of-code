use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};

const STARTING_POSITION: i32 = 50;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, directions) = directions.parse(input).unwrap();

    let (_final_position, counter) =
        directions
            .iter()
            .fold((STARTING_POSITION, 0), |(dial_position, counter), direction| {
                let turns = match direction {
                    Direction::Left(turns) => -turns,
                    Direction::Right(turns) => *turns,
                };

                let (next_dial_position, revolutions) = spin_dial(dial_position, turns);
                (next_dial_position, counter + revolutions)
            });

    Ok(counter.to_string())
}

fn spin_dial(dial_position: i32, turns: i32) -> (i32, i32) {
    let dial_long = dial_position + turns;
    let mut revolutions = (dial_long / 100).abs();

    if dial_position != 0 && dial_long <= 0 {
        revolutions += 1;
    }

    (dial_long.rem_euclid(100), revolutions)
}

#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, direction).parse(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((tag("L"), tag("R"))).parse(input)?;
    let (input, turns) = complete::i32(input)?;

    Ok((
        input,
        match direction {
            "L" => Direction::Left(turns),
            "R" => Direction::Right(turns),
            x => unimplemented!("unknown {x}"),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!("6", process(input)?);
        Ok(())
    }
}
