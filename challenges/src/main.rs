pub mod day_1;
pub mod day_2;

use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Error};
use aoc_core::all_challenges;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let args = Command::from_args();

    match args {
        Command::Run { challenge, input } => {
            let input = match input {
                Some(filename) => {
                    let f = File::open(&filename).with_context(|| {
                        format!("unable to open \"{}\"", filename.display())
                    })?;
                    Box::new(f) as Box<dyn Read>
                }
                None => Box::new(std::io::stdin()) as Box<dyn Read>,
            };

            run_challenge(input, &challenge)?;
        }
        Command::List => list_challenges(),
    }

    Ok(())
}

#[derive(StructOpt)]
enum Command {
    #[structopt(about = "Run a particular challenge")]
    Run {
        #[structopt(help = "The challenge to run")]
        challenge: String,
        #[structopt(
            short,
            long,
            parse(from_os_str),
            help = "A file to read input from (stdin if not provided)"
        )]
        input: Option<PathBuf>,
    },
    #[structopt(about = "Print all known challenges")]
    List,
}

fn run_challenge<R: Read>(mut reader: R, challenge: &str) -> Result<(), Error> {
    let challenge = all_challenges()
        .find(|c| c.number == challenge)
        .context("Unknown challenge number")?;

    let mut input = Vec::new();
    reader
        .read_to_end(&mut input)
        .context("Unable to read the full input")?;

    let input = String::from_utf8(input)
        .context("Unable to read the input as UTF-8 text")?;

    let output = (challenge.solve)(&input)?;
    println!("{}", output);

    Ok(())
}

fn list_challenges() {
    let mut challenges: Vec<_> = aoc_core::all_challenges().collect();
    challenges.sort_by_key(|c| c.number);

    for challenge in challenges {
        println!("{}: {}", challenge.number, challenge.name);
    }
}
