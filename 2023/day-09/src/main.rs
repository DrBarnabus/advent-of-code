mod part1;
mod part2;

fn main() {
    let input = include_str!("input.txt");
    println!("Input - Part 1: {}", part1::process(input));
    println!("Input - Part 2: {}", part2::process(input));
}
