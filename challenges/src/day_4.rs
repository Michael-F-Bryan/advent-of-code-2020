use std::{borrow::Borrow, hash::Hash};
use std::{collections::HashMap, convert::TryFrom, ops::Deref, str::FromStr};

use anyhow::{Context, Error};

/// Day 4a: Passport Processing
///
/// # Description
///
/// You arrive at the airport only to realize that you grabbed your North Pole
/// Credentials instead of your passport. While these documents are extremely
/// similar, North Pole Credentials aren't issued by a country and therefore
/// aren't actually valid documentation for travel in most of the world.
///
/// It seems like you're not the only one having problems, though; a very long
/// line has formed for the automatic passport scanners, and the delay could
/// upset your travel itinerary.
///
/// Due to some questionable network security, you realize you might be able to
/// solve both of these problems at the same time.
///
/// The automatic passport scanners are slow because they're having trouble
/// detecting which passports have all required fields. The expected fields are
/// as follows:
///
/// ```text
/// byr (Birth Year)
/// iyr (Issue Year)
/// eyr (Expiration Year)
/// hgt (Height)
/// hcl (Hair Color)
/// ecl (Eye Color)
/// pid (Passport ID)
/// cid (Country ID)
/// ```
///
/// Passport data is validated in batch files (your puzzle input). Each passport
/// is represented as a sequence of key:value pairs separated by spaces or
/// newlines. Passports are separated by blank lines.
///
/// Here is an example batch file containing four passports:
///
/// ```text
/// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
/// byr:1937 iyr:2017 cid:147 hgt:183cm
///
/// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
/// hcl:#cfa07d byr:1929
///
/// hcl:#ae17e1 iyr:2013
/// eyr:2024
/// ecl:brn pid:760753108 byr:1931
/// hgt:179cm
///
/// hcl:#cfa07d eyr:2025 pid:166559648
/// iyr:2011 ecl:brn hgt:59in
/// ```
///
/// The first passport is valid - all eight fields are present. The second
/// passport is invalid - it is missing hgt (the Height field).
///
/// The third passport is interesting; the only missing field is cid, so it looks
/// like data from North Pole Credentials, not a passport at all! Surely, nobody
/// would mind if you made the system temporarily ignore missing cid fields.
/// Treat this "passport" as valid.
///
/// The fourth passport is missing two fields, cid and byr. Missing cid is fine,
/// but missing any other field is not, so this passport is invalid.
///
/// According to the above rules, your improved system would report 2 valid
/// passports.
///
/// Count the number of valid passports - those that have all required fields.
/// Treat cid as optional. In your batch file, how many passports are valid?
#[aoc_macros::challenge]
pub fn part_1(passports: Passports<'_>) -> Result<usize, Error> {
    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    Ok(passports
        .iter()
        .filter(|p| {
            required_fields
                .iter()
                .all(|field_name| p.contains_key(field_name))
        })
        .count())
}

/// Day 4b: Passport Processing (part 2)
///
/// # Description
///
/// The line is moving more quickly now, but you overhear airport security
/// talking about how passports with invalid data are getting through. Better add
/// some data validation, quick!
///
/// You can continue to ignore the cid field, but each other field has strict
/// rules about what values are valid for automatic validation:
///
/// ```text
/// byr (Birth Year) - four digits; at least 1920 and at most 2002.
/// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
/// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
/// hgt (Height) - a number followed by either cm or in:
///     If cm, the number must be at least 150 and at most 193.
///     If in, the number must be at least 59 and at most 76.
/// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
/// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
/// pid (Passport ID) - a nine-digit number, including leading zeroes.
/// cid (Country ID) - ignored, missing or not.
/// ```
///
/// Your job is to count the passports where all required fields are both present
/// and valid according to the above rules. Here are some example values:
///
/// ```text
/// byr valid:   2002
/// byr invalid: 2003
///
/// hgt valid:   60in
/// hgt valid:   190cm
/// hgt invalid: 190in
/// hgt invalid: 190
///
/// hcl valid:   #123abc
/// hcl invalid: #123abz
/// hcl invalid: 123abc
///
/// ecl valid:   brn
/// ecl invalid: wat
///
/// pid valid:   000000001
/// pid invalid: 0123456789
/// ```
///
/// Here are some invalid passports:
///
/// ```text
/// eyr:1972 cid:100
/// hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
///
/// iyr:2019
/// hcl:#602927 eyr:1967 hgt:170cm
/// ecl:grn pid:012533040 byr:1946
///
/// hcl:dab227 iyr:2012
/// ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
///
/// hgt:59cm ecl:zzz
/// eyr:2038 hcl:74454a iyr:2023
/// pid:3556412378 byr:2007
/// ```
///
/// Here are some valid passports:
///
/// ```text
/// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
/// hcl:#623a2f
///
/// eyr:2029 ecl:blu cid:129 byr:1989
/// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
///
/// hcl:#888785
/// hgt:164cm byr:2001 iyr:2015 cid:88
/// pid:545766238 ecl:hzl
/// eyr:2022
///
/// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
/// ```
///
/// Count the number of valid passports - those that have all required fields and
/// valid values. Continue to treat cid as optional. In your batch file, how many
/// passports are valid?
#[aoc_macros::challenge]
pub fn part_2(passports: Passports<'_>) -> Result<usize, Error> {
    Ok(passports.iter().filter(|p| is_valid(p)).count())
}

fn is_valid(passport: &Passport<'_>) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.

    // Note: This was massively over-engineered, using a pseudo-monad approach

    true && check(&passport.fields)
        .and_then(require_key("byr"))
        .and_then(is_digit)
        .and_then(between(1920, 2002))
        .is_some()
        && check(&passport.fields)
            .and_then(require_key("iyr"))
            .and_then(is_digit)
            .and_then(between(2010, 2020))
            .is_some()
        && check(&passport.fields)
            .and_then(require_key("eyr"))
            .and_then(is_digit)
            .and_then(between(2020, 2030))
            .is_some()
        && check(&passport.fields)
            .and_then(require_key("hgt"))
            .and_then(parse::<Height, _>)
            .and_then(validate_height)
            .is_some()
        && check(&passport.fields)
            .and_then(require_key("hcl"))
            .and_then(parse::<Colour, _>)
            .is_some()
        && check(&passport.fields)
            .and_then(require_key("ecl"))
            .copied()
            .and_then(is_one_of::<&str, _>([
                "amb", "blu", "brn", "gry", "grn", "hzl", "oth",
            ]))
            .is_some()
        && check(&passport.fields)
            .and_then(require_key("pid"))
            .and_then(decimal_number_with_length(9))
            .is_some()
}

pub struct Colour(u32);

impl FromStr for Colour {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::ensure!(s.starts_with("#"));
        let number = &s[1..];
        anyhow::ensure!(number.len() == 6);

        let hex = u32::from_str_radix(number, 16)?;

        Ok(Colour(hex))
    }
}

pub enum Height {
    Centimeters(u32),
    Inches(u32),
}

impl FromStr for Height {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (number, f) = match s.as_bytes() {
            [start @ .., b'c', b'm'] => {
                (start, Height::Centimeters as fn(u32) -> Height)
            }
            [start @ .., b'i', b'n'] => {
                (start, Height::Inches as fn(u32) -> Height)
            }
            _ => {
                return Err(Error::msg(
                    "Expected a height like \"150cm\" or \"90in\"",
                ));
            }
        };

        let number = std::str::from_utf8(number)
            .expect("Guaranteed to be valid")
            .parse()?;

        Ok(f(number))
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Passports<'input>(Vec<Passport<'input>>);

impl<'input> Deref for Passports<'input> {
    type Target = Vec<Passport<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'input> TryFrom<&'input str> for Passports<'input> {
    type Error = Error;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let mut passports = Vec::new();

        let mut current_passport = Passport::default();
        let mut line_number = 0;

        for line in value.lines() {
            line_number += 1;

            if line.is_empty() {
                passports.push(std::mem::take(&mut current_passport));
                continue;
            }

            for pair in line.split_whitespace() {
                let colon = pair.find(":").with_context(|| {
                    format!(
                        "Expected \"{}\" on line {} to look like \"key:value\"",
                        pair, line_number
                    )
                })?;

                let (key, value) = pair.split_at(colon);
                let value = &value[1..];
                current_passport.fields.insert(key, value);
            }
        }

        if !current_passport.is_empty() {
            passports.push(current_passport);
        }

        Ok(Passports(passports))
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Passport<'input> {
    fields: HashMap<&'input str, &'input str>,
}

impl<'input> Deref for Passport<'input> {
    type Target = HashMap<&'input str, &'input str>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

pub fn require_key<S, T>(
    key: &'static str,
) -> impl for<'a> Fn(&'a HashMap<S, T>) -> Option<&'a T>
where
    S: Borrow<str> + Eq + Hash,
{
    move |map| map.get(&key)
}

pub fn is_digit<S>(text: S) -> Option<u32>
where
    S: AsRef<str>,
{
    text.as_ref().parse().ok()
}

pub fn between<T>(min: T, max: T) -> impl Fn(T) -> Option<()>
where
    T: PartialOrd + 'static,
{
    move |value| predicate(min <= value && value <= max)
}

pub fn check<T>(value: T) -> Option<T> {
    Some(value)
}

pub fn is_one_of<T, V>(values: V) -> impl Fn(T) -> Option<()>
where
    T: PartialEq,
    V: AsRef<[T]>,
{
    move |value| {
        predicate(values.as_ref().iter().any(|candidate| *candidate == value))
    }
}

fn predicate(value: bool) -> Option<()> {
    if value {
        Some(())
    } else {
        None
    }
}

pub fn parse<T, S>(text: S) -> Option<T>
where
    S: AsRef<str>,
    T: FromStr,
{
    text.as_ref().parse().ok()
}

pub fn decimal_number_with_length<S>(length: usize) -> impl Fn(S) -> Option<u32>
where
    S: AsRef<str>,
{
    move |word| {
        let word = word.as_ref();

        if word.len() != length {
            return None;
        }

        word.parse().ok()
    }
}

fn validate_height(height: Height) -> Option<()> {
    let is_valid = match height {
        Height::Centimeters(value) => 150 <= value && value <= 193,
        Height::Inches(value) => 59 <= value && value <= 76,
    };

    predicate(is_valid)
}
