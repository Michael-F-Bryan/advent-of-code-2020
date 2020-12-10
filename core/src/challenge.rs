use anyhow::Error;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Example {
    pub input: &'static str,
    pub expected: &'static str,
}

/// Iterate over all the challenges registered with the
/// `#[aoc_macros::challenge]` macro.
pub fn all_challenges() -> impl Iterator<Item = &'static Challenge> {
    inventory::iter::<Challenge>.into_iter()
}

#[derive(Copy, Clone)]
pub struct Challenge {
    pub number: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub examples: &'static [Example],
    pub solve: fn(&str) -> Result<String, Error>,
}

inventory::collect!(Challenge);

impl Debug for Challenge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Challenge {
            number: day,
            name,
            description,
            examples,
            ..
        } = self;

        f.debug_struct("Challenge")
            .field("day", day)
            .field("name", name)
            .field("description", description)
            .field("examples", examples)
            .finish()
    }
}
