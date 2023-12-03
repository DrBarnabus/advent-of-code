use std::collections::BTreeMap;
use itertools::Itertools;

#[derive(Debug)]
enum Value { Empty, Symbol(char), Number(u32) }

pub fn process(input: &str) -> String {
    // y, x
    let map = create_map_from_input(input);

    // x, y
    let number_groups = extract_number_groups(&map);

    let mut total = 0;
    for symbol in map.iter().filter(|(position, value)| matches!(value, Value::Symbol('*'))) {
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

        let positions_to_check: Vec<(i32, i32)> = POSITION_OFFSETS
            .iter()
            .map(|offset| (offset.0 + symbol.0.1, offset.1 + symbol.0.0))
            .collect();

        let mut indexes_of_numbers = vec![];
        for position in positions_to_check {
            for (i, number_group) in number_groups.iter().enumerate() {
                if number_group.iter().find(|(number_position, _)| *number_position == position).is_some() {
                    indexes_of_numbers.push(i);
                }
            }
        }

        if indexes_of_numbers.iter().unique().count() == 2 {
            total += indexes_of_numbers
                .iter()
                .unique()
                .map(|i| {
                    number_groups[*i]
                        .iter()
                        .map(|(_, number)| number.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>();
        }
    }

    total.to_string()
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
        assert_eq!(result, "467835");
    }
}
