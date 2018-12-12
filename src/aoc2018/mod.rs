
mod day01;

use crate::errors::Result;

pub fn run(day: u16) -> Result<()> {
    match day {
        1 => day01::run(),
        _ => bail!("Unknown day {}", day)
    }
    //Ok(())
}