mod errors;
mod aoc2018;

#[macro_use] extern crate failure;

use crate::errors::Result;
//use failure;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Run Advent of Code task for the specific year and day.")]
struct Opt {
    /// Advent of Code year.
    #[structopt(short = "y", long = "year", default_value = "2018")]
    year: u16,
    /// Pattern string for the file name (rust regex).
    #[structopt(short = "d", long = "day")]
    day: u16,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.year {
        2018 => aoc2018::run(opt.day),
        _ => bail!("Unknown year {}", opt.year)
    }
}
