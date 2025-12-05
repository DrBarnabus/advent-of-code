#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    todo!("part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        todo!("test");
        let input = "";

        assert_eq!("", process(input)?);
        Ok(())
    }
}
