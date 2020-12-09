use anyhow::{Context, Error};
use aoc_core::Lines;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

/// Day 2a: Password Philosophy
///
/// # Description
///
/// Your flight departs in a few days from the coastal airport; the easiest way
/// down to the coast from here is via toboggan.
///
/// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
/// "Something's wrong with our computers; we can't log in!" You ask if you can
/// take a look.
///
/// Their password database seems to be a little corrupted: some of the passwords
/// wouldn't have been allowed by the Official Toboggan Corporate Policy that was
/// in effect when they were chosen.
///
/// To try to debug the problem, they have created a list (your puzzle input) of
/// passwords (according to the corrupted database) and the corporate policy when
/// that password was set.
///
/// For example, suppose you have the following list:
///
/// ```
/// 1-3 a: abcde
/// 1-3 b: cdefg
/// 2-9 c: ccccccccc
/// ```
///
/// Each line gives the password policy and then the password. The password
/// policy indicates the lowest and highest number of times a given letter must
/// appear for the password to be valid. For example, 1-3 a means that the
/// password must contain a at least 1 time and at most 3 times.
///
/// In the above example, 2 passwords are valid. The middle password, cdefg, is
/// not; it contains no instances of b, but needs at least 1. The first and third
/// passwords are valid: they contain one a or nine c, both within the limits of
/// their respective policies.
///
/// How many passwords are valid according to their policies?
#[aoc_macros::challenge]
pub fn part_1(lines: Lines<Input>) -> Result<usize, Error> {
    Ok(lines
        .into_iter()
        .filter(|Input { rule, password }| {
            first_password_rule_is_valid(*rule, password)
        })
        .count())
}

/// Day 2b: Password Philosophy
///
/// # Description
///
/// While it appears you validated the passwords correctly, they don't seem to be
/// what the Official Toboggan Corporate Authentication System is expecting.
///
/// The shopkeeper suddenly realizes that he just accidentally explained the
/// password policy rules from his old job at the sled rental place down the
/// street! The Official Toboggan Corporate Policy actually works a little
/// differently.
///
/// Each policy actually describes two positions in the password, where 1 means
/// the first character, 2 means the second character, and so on. (Be careful;
/// Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of
/// these positions must contain the given letter. Other occurrences of the
/// letter are irrelevant for the purposes of policy enforcement.
///
/// Given the same example list from above:
///
/// ```
/// 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
/// 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
/// 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
/// ```
///
/// How many passwords are valid according to the new interpretation of the
/// policies?
#[aoc_macros::challenge]
pub fn part_2(lines: Lines<Input>) -> Result<usize, Error> {
    Ok(lines
        .into_iter()
        .filter(|Input { rule, password }| {
            second_password_rule_is_valid(*rule, password)
        })
        .count())
}

fn first_password_rule_is_valid(rule: Rule, password: &str) -> bool {
    let Rule {
        letter,
        a: min,
        b: max,
    } = rule;

    let occurrences = password.chars().filter(|c| *c == letter).count();

    min <= occurrences && occurrences <= max
}

fn second_password_rule_is_valid(rule: Rule, password: &str) -> bool {
    let Rule { letter, a, b } = rule;

    let letters: Vec<_> = password.chars().collect();

    (letters[a - 1] == letter) ^ (letters[b - 1] == letter)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    pub rule: Rule,
    pub password: String,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(":").context(
            "Expected the rule and password to be separated by a colon",
        )?;
        let (rule, password) = s.split_at(colon);
        let password = &password[1..];

        Ok(Input {
            rule: rule
                .trim()
                .parse()
                .context(r#"Rules should look like "2-15 x""#)?,
            password: password.trim().to_string(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rule {
    pub a: usize,
    pub b: usize,
    pub letter: char,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\d+)-(\d+)\s*(\w)$").unwrap());

        let captures = PATTERN
            .captures(s)
            .ok_or_else(|| Error::msg("Unable to parse the password rule"))?;

        let a = captures[1]
            .parse()
            .context("Couldn't parse the first value")?;
        let b = captures[2]
            .parse()
            .context("Couldn't parse the second value")?;

        let mut letters = captures[3].chars();
        let letter =
            letters.next().expect("Regex guarantees at least 1 letter");
        anyhow::ensure!(
            letters.next().is_none(),
            "The rule should only include one letter"
        );

        Ok(Rule { a, b, letter })
    }
}
