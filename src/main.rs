
use failure::{Error, ResultExt};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Run Advent of Code task for the specific year and day.")]
struct Opt {
    /// Advent of Code year.
    #[structopt(short = "y", long = "year")]
    year: u16,
    /// Pattern string for the file name (rust regex).
    #[structopt(short = "d", long = "day")]
    day: u16,
    // /// Pattern string for the text (rust regex).
    // #[structopt(parse(from_str))]
    // text_pattern: String,
    // /// Replacement string (rust regex). Do only pattern matching if not specified.
    // #[structopt(short = "r", long = "replace")]
    // replacement: Option<String>,
    // /// Input files and/or starting directories. Searches in the current directory if not specified.
    // #[structopt(parse(from_os_str))]
    // inputs: Vec<PathBuf>,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    println!("Hello, world!");

    Ok(())
}
