use std::collections::HashSet;

use glam::IVec2;

const NEIGHBORS: [IVec2; 8] = [
    IVec2::new(1, 0),   // Right
    IVec2::new(1, -1),  // Down Right
    IVec2::new(0, -1),  // Down
    IVec2::new(-1, -1), // Down Left
    IVec2::new(-1, 0),  // Left
    IVec2::new(-1, 1),  // Up Left
    IVec2::new(0, 1),   // Up
    IVec2::new(1, 1),   // Up Right
];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut positions = parse(input);

    let mut total_removed = 0;
    loop {
        let to_remove: HashSet<IVec2> = positions
            .iter()
            .filter(|&position| {
                NEIGHBORS
                    .iter()
                    .filter(|&offset| positions.contains(&(position + offset)))
                    .count()
                    < 4
            })
            .cloned()
            .collect();

        if to_remove.len() == 0 {
            break;
        } else {
            total_removed += to_remove.len();
        }

        positions = positions.difference(&to_remove).cloned().collect();
    }

    Ok(total_removed.to_string())
}

fn parse(input: &str) -> HashSet<IVec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, value)| (value == '@').then_some(IVec2::new(x as i32, y as i32)))
        })
        .collect::<HashSet<IVec2>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!("43", process(input)?);
        Ok(())
    }
}
