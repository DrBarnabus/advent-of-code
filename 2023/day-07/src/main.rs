mod part1;
mod part2;

fn main() {
    let example = include_str!("example.txt");
    println!("Example - Part 1: {}", part1::process(example));
    println!("Example - Part 2: {}", part2::process(example));

    let input = include_str!("input.txt");
    println!("Input - Part 1: {}", part1::process(input));
    println!("Input - Part 2: {}", part2::process(input));
}
