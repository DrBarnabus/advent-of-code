use itertools::Itertools;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let total = input
        .lines()
        .map(|bank| {
            let (index, first_max) = &bank[..(bank.len() - 1)]
                .chars()
                .enumerate()
                .max_set_by_key(|(_index, battery)| *battery)
                .first()
                .cloned()
                .unwrap();

            let (_, second_max) = &bank[(index + 1)..]
                .chars()
                .enumerate()
                .max_set_by_key(|(_index, battery)| *battery)
                .first()
                .cloned()
                .unwrap();

            info!(bank, index, ?first_max, ?second_max);

            str::parse::<u64>(format!("{first_max}{second_max}").as_str()).unwrap()
        })
        .sum::<u64>();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!("357", process(input)?);
        Ok(())
    }
}
