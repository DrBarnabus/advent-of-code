fn main() {
    let example = include_str!("example");
    dbg!(part1(example).to_string());
    dbg!(part2(example).to_string());

    let input = include_str!("input.txt");
    dbg!(part1(input).to_string());
    dbg!(part2(input).to_string());
}

#[derive(Debug)]
struct Round {
    r: u32,
    g: u32,
    b: u32,
    possible: bool
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let split_by_colon: Vec<&str> = line
                .split(':')
                .collect();

            let id = split_by_colon[0]
                .replace("Game ", "")
                .parse::<u32>()
                .expect("Id of game should be a number");

            let rounds: Vec<Round> = split_by_colon[1]
                .split(';')
                .map(|round_data| {
                    let mut round = Round { r: 0, g: 0, b: 0, possible: false };
                    for round_data_point in round_data.trim().split(", ") {
                        let split_round_data_point: Vec<&str> = round_data_point.split(' ').collect();
                        match split_round_data_point[1] {
                            "red" => {
                                round.r = split_round_data_point[0].parse::<u32>().expect("Expected a u32");
                            }
                            "green" => {
                                round.g = split_round_data_point[0].parse::<u32>().expect("Expected a u32");
                            },
                            "blue" => {
                                round.b = split_round_data_point[0].parse::<u32>().expect("Expected a u32");
                            }
                            _ => {}
                        }
                    }

                    match round {
                        Round { r, g, b, possible: _ } if r <= 12 && g <= 13 && b <= 14 => {
                            round.possible = true },
                        _ => {}
                    }

                    round
                })
                .collect();

            (id, rounds)
        })
        .filter_map(|game| {
            if game.1.iter().filter(|r| !r.possible).count() == 0 {
                Some(game.0)
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let data: &str = line.split(':').last().expect("Should contain data after the :");

            let rounds: Vec<Round> = data
                .split(';')
                .map(|round_data| {
                    let mut round = Round { r: 0, g: 0, b: 0, possible: false };
                    for round_data_point in round_data.trim().split(", ") {
                        let split_round_data_point: Vec<&str> = round_data_point.split(' ').collect();
                        match split_round_data_point[1] {
                            "red" => {
                                round.r = split_round_data_point[0].parse::<u32>().expect("Expected a u32");
                            }
                            "green" => {
                                round.g = split_round_data_point[0].parse::<u32>().expect("Expected a u32");
                            },
                            "blue" => {
                                round.b = split_round_data_point[0].parse::<u32>().expect("Expected a u32");
                            }
                            _ => {}
                        }
                    }

                    round
                })
                .collect();

            let min_r = rounds.iter().max_by_key(|r| r.r).unwrap().r;
            let min_g = rounds.iter().max_by_key(|r| r.g).unwrap().g;
            let min_b = rounds.iter().max_by_key(|r| r.b).unwrap().b;

            min_r * min_g * min_b
        })
        .sum::<u32>()
}
