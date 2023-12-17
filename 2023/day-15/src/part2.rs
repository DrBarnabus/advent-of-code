use nom::{
    IResult,
    combinator::opt,
    character::{
        complete::alpha1,
        complete
    },
    bytes::complete::is_a,
    multi::separated_list1,
};

#[derive(Debug)]
enum Operation {
    Insert(u8),
    Remove,
}

#[derive(Debug)]
struct Instruction<'a> {
    label: &'a str,
    operation: Operation,
    hash: u8,
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

impl<'a> TryFrom<&Instruction<'a>> for Lens<'a> {
    type Error = String;

    fn try_from(value: &Instruction<'a>) -> Result<Self, Self::Error> {
        match value.operation {
            Operation::Insert(focal_length) => Ok(Lens { label: value.label, focal_length }),
            Operation::Remove => Err(format!("unable to convert label {}", value.label))
        }
    }
}

pub fn process(input: &str) -> String {
    let (_, instructions) = parse_instructions(input).expect("valid parsed data");

    let boxes: Vec<Vec<Lens>> = (0..256).map(|_| vec![]).collect();
    let filled_boxes = instructions
        .iter()
        .fold(boxes, |mut boxes, next_instruction| {
            match Lens::try_from(next_instruction) {
                Ok(lens) => {
                    let index = boxes[next_instruction.hash as usize].iter().position(|a| a.label == lens.label);
                    match index {
                        Some(lens_index) => {
                            let _ = std::mem::replace(&mut boxes[next_instruction.hash as usize][lens_index], lens);
                        }
                        None => {
                            boxes[next_instruction.hash as usize].push(lens);
                        }
                    }
                }
                Err(_) => {
                    let r#box = &mut boxes[next_instruction.hash as usize];
                    r#box.retain(|lens| lens.label != next_instruction.label)
                }
            }

            boxes
        });

    filled_boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_position, r#box)| {
            r#box.into_iter().enumerate().map(move |(lens_position, lens)| {
                (box_position + 1) * (lens_position + 1) * (lens.focal_length as usize)
            })
        })
        .sum::<usize>()
        .to_string()
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, label) = alpha1(input)?;
    let (input, _operation) = is_a("-=")(input)?;
    let (input, focal_length) = opt(complete::u8)(input)?;

    let operation = match focal_length {
        Some(number) => Operation::Insert(number),
        None => Operation::Remove,
    };

    Ok((input, Instruction {
        label,
        operation,
        hash: label
            .chars()
            .fold(0, |accumulator, next_char| (accumulator + (next_char as usize)) * 17 % 256)
            .try_into()
            .expect("should result in a u8"),
    }))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(complete::char(','), parse_instruction)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(include_str!("example.txt"));
        assert_eq!(result, "145");
    }
}