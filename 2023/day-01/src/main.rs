fn main() {
    let example1 = include_str!("part1-example.txt");
    dbg!(part1(example1).to_string());

    let example2 = include_str!("part2-example.txt");
    dbg!(part2(example2).to_string());

    let input = include_str!("input.txt");
    dbg!(part1(input).to_string());
    dbg!(part2(input).to_string());
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));

            let first_digit = digits.next().expect("should contain at least 1 digit");
            match digits.last() {
                Some(second_digit) => format!("{first_digit}{second_digit}"),
                None => format!("{first_digit}{first_digit}")
            }
            .parse::<u32>()
            .expect("should be a valid u32")
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input.lines().map(process_line).sum::<u32>()
}

fn process_line(line: &str) -> u32 {
    let mut digits = (0..line.len()).filter_map(|i| {
        let partial_line = &line[i..];

        let result = if partial_line.starts_with("one") {
            '1'
        } else if partial_line.starts_with("two") {
            '2'
        } else if partial_line.starts_with("three") {
            '3'
        } else if partial_line.starts_with("four") {
            '4'
        } else if partial_line.starts_with("five") {
            '5'
        } else if partial_line.starts_with("six") {
            '6'
        } else if partial_line.starts_with("seven") {
            '7'
        } else if partial_line.starts_with("eight") {
            '8'
        } else if partial_line.starts_with("nine") {
            '9'
        } else {
            partial_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });

    let first_digit = digits.next().expect("should contain at least 1 digit");
    match digits.last() {
        Some(second_digit) => format!("{first_digit}{second_digit}"),
        None => format!("{first_digit}{first_digit}")
    }
        .parse::<u32>()
        .expect("should be a valid u32")
}
