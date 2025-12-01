#!/usr/bin/env cargo +nightly -Zscript
---cargo
[package]
edition = "2024"
[dependencies]
clap = { version = "4.5", features = ["derive"] }
nom = "8"
reqwest = { version = "0.12", features=["blocking"] }
---

use clap::{error::ErrorKind, CommandFactory, Parser};
use nom::{bytes::complete::tag, character::complete, sequence::preceded, IResult, Parser as NomParser};
use reqwest::{blocking::Client, header::COOKIE};
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long)]
    day: String,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32).parse(input)
}

fn main() -> Result<(), reqwest::Error> {
    let session = std::env::var("SESSION").expect("should have a session token set");
    let args = Args::parse();

    let Ok((_, day)) = parse_day(&args.day) else {
        let mut cmd = Args::command();
        cmd.error(ErrorKind::ValueValidation, format!("day `{}` must be formatted as `day-01`", args.day))
            .exit()
    };

    let current_dir = std::env::current_dir().expect("should be able to get current directory");

    let client = Client::new();
    let input_data = client
        .get(format!("https://adventofcode.com/2025/day/{day}/input"))
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;

    for filename in ["input1.txt", "input2.txt"] {
        let file_path = current_dir.join(&args.day).join(filename);
        let mut file = File::create(&file_path).expect("should be able to create a file");

        file.write_all(input_data.as_bytes()).expect("should be able to write to input file");
        println!("wrote {}", file_path.display());
    }

    Ok(())
}
