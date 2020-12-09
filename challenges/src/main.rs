pub mod day_1;
pub mod day_2;

use std::io::Read;

use anyhow::{Context, Error};
use aoc_core::all_challenges;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let args = Command::from_args();

    match args {
        Command::Run { challenge } => run_challenge(&challenge),
    }
}

#[derive(StructOpt)]
enum Command {
    Run {
        #[structopt(help = "The challenge to run")]
        challenge: String,
    },
}

fn run_challenge(challenge: &str) -> Result<(), Error> {
    let challenge = all_challenges()
        .find(|c| c.number == challenge)
        .context("Unknown challenge number")?;

    let mut input = Vec::new();
    std::io::stdin()
        .read_to_end(&mut input)
        .context("Unable to read the full input")?;

    let input = String::from_utf8(input)
        .context("Unable to read the input as UTF-8 text")?;

    let output = (challenge.solve)(&input)?;
    println!("{}", output);

    Ok(())
}
