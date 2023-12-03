use std::collections::BTreeMap;
use itertools::Itertools;

#[derive(Debug)]
enum Value { Empty, Symbol(char), Number(u32) }

pub fn process(input: &str) -> String {
    // y, x
    let map = create_map_from_input(input);

    // x, y
    let number_groups = extract_number_groups(&map);

    let total = calculate_total(map, number_groups);

    total.to_string()
}

fn calculate_total(map: BTreeMap<(i32, i32), Value>, number_groups: Vec<Vec<((i32, i32), u32)>>) -> u32 {
    let mut total = 0;
    for number_group in number_groups {
        // x, y
        const POSITION_OFFSETS: [(i32, i32); 8] = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let number_positions: Vec<(i32, i32)> = number_group.iter().map(|((x, y), _)| (*x, *y)).collect();

        let is_part_number = number_group
            .iter()
            .flat_map(|(position, _)| {
                POSITION_OFFSETS.iter().map(|offset| {
                    (
                        offset.0 + position.0,
                        offset.1 + position.1
                    )
                })
            })
            .unique()
            .filter(|position| !number_positions.contains(position))
            .any(|(x, y)| matches!(map.get(&(y, x)), Some(Value::Symbol(_))));

        if is_part_number {
            total += number_group
                .iter()
                .map(|(_, number)| number.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
    }

    total
}

fn extract_number_groups(map: &BTreeMap<(i32, i32), Value>) -> Vec<Vec<((i32, i32), u32)>> {
    let mut number_groups: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(number) = value {
            match number_groups.iter().last() {
                Some(v) => {
                    let last_number = v.iter().last();

                    match last_number {
                        Some(((last_number_x, _), _)) => {
                            if last_number_x + 1 == *x {
                                let last_number_group = number_groups.iter_mut().last().expect("last inserted should exist");
                                last_number_group.push(((*x, *y), *number));
                            } else {
                                number_groups.push(vec![((*x, *y), *number)]);
                            }
                        }
                        None => unreachable!()
                    }
                }
                None => {
                    number_groups.push(vec![((*x, *y), *number)]);
                }
            }
        }
    }

    number_groups
}

fn create_map_from_input(input: &str) -> BTreeMap<(i32, i32), Value> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                ((y as i32, x as i32), match character {
                    '.' => Value::Empty,
                    c if c.is_ascii_digit() => {
                        Value::Number(c.to_digit(10).expect("should be a number"))
                    }
                    c => Value::Symbol(c)
                })
            })
        })
        .collect::<BTreeMap<(i32, i32), Value>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "4361");
    }
}
