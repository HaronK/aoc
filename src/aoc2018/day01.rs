use crate::errors::Result;
use std::io::{self, Read};

pub fn run() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    println!("Test: {}", input);
    Ok(())
}
