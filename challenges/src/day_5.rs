use anyhow::Error;
use aoc_core::Lines;
use std::{ops::Range, str::FromStr};

/// Day 5a: Binary Boarding (part 1)
///
/// # Description
///
/// You board your plane only to discover a new problem: you dropped your
/// boarding pass! You aren't sure which seat is yours, and all of the flight
/// attendants are busy with the flood of people that suddenly made it through
/// passport control.
///
/// You write a quick program to use your phone's camera to scan all of the
/// nearby boarding passes (your puzzle input); perhaps you can find your seat
/// through process of elimination.
///
/// Instead of zones or groups, this airline uses binary space partitioning to
/// seat people. A seat might be specified like FBFBBFFRLR, where F means
/// "front", B means "back", L means "left", and R means "right".
///
/// The first 7 characters will either be F or B; these specify exactly one of
/// the 128 rows on the plane (numbered 0 through 127). Each letter tells you
/// which half of a region the given seat is in. Start with the whole list of
/// rows; the first letter indicates whether the seat is in the front (0 through
/// 63) or the back (64 through 127). The next letter indicates which half of
/// that region the seat is in, and so on until you're left with exactly one row.
///
/// For example, consider just the first seven characters of FBFBBFFRLR:
///
/// ```text
/// Start by considering the whole range, rows 0 through 127.
/// F means to take the lower half, keeping rows 0 through 63.
/// B means to take the upper half, keeping rows 32 through 63.
/// F means to take the lower half, keeping rows 32 through 47.
/// B means to take the upper half, keeping rows 40 through 47.
/// B keeps rows 44 through 47.
/// F keeps rows 44 through 45.
/// The final F keeps the lower of the two, row 44.
/// ```
///
/// The last three characters will be either L or R; these specify exactly one of
/// the 8 columns of seats on the plane (numbered 0 through 7). The same process
/// as above proceeds again, this time with only three steps. L means to keep the
/// lower half, while R means to keep the upper half.
///
/// For example, consider just the last 3 characters of FBFBBFFRLR:
///
/// ```text
/// Start by considering the whole range, columns 0 through 7.
/// R means to take the upper half, keeping columns 4 through 7.
/// L means to take the lower half, keeping columns 4 through 5.
/// The final R keeps the upper of the two, column 5.
/// ```
///
/// So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
///
/// Every seat also has a unique seat ID: multiply the row by 8, then add the
/// column. In this example, the seat has ID 44 * 8 + 5 = 357.
///
/// Here are some other boarding passes:
///
/// ```text
/// BFFFBBFRRR: row 70, column 7, seat ID 567.
/// FFFBBBFRRR: row 14, column 7, seat ID 119.
/// BBFFBBFRLL: row 102, column 4, seat ID 820.
/// ```
///
/// As a sanity check, look through your list of boarding passes. What is the
/// highest seat ID on a boarding pass?
#[aoc_macros::challenge]
pub fn part_1(boarding_passes: Lines<BoardingPass>) -> Result<u32, Error> {
    boarding_passes
        .iter()
        .map(|b| b.location().id())
        .max()
        .ok_or_else(|| Error::msg("No boarding passes provided"))
}

/// Day 5b: Binary Boarding (part 2)
///
/// # Description
///
/// Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
///
/// It's a completely full flight, so your seat should be the only missing
/// boarding pass in your list. However, there's a catch: some of the seats at
/// the very front and back of the plane don't exist on this aircraft, so they'll
/// be missing from your list as well.
///
/// Your seat wasn't at the very front or back, though; the seats with IDs +1 and
/// -1 from yours will be in your list.
///
/// What is the ID of your seat?
#[aoc_macros::challenge]
pub fn part_2(boarding_passes: Lines<BoardingPass>) -> Result<u32, Error> {
    let mut seat_ids: Vec<_> =
        boarding_passes.iter().map(|b| b.location().id()).collect();
    seat_ids.sort();

    for window in seat_ids.windows(2) {
        let first = window[0];
        let second = window[1];

        let candidate = first + 1;

        if candidate != second {
            return Ok(candidate);
        }
    }

    Err(Error::msg("Unable to find the seat number"))
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoardingPass {
    rows: Vec<Direction>,
    seats: Vec<Direction>,
}

impl BoardingPass {
    pub fn location(&self) -> Seat {
        let row = partition_range(&self.rows, 0..128);
        let seat = partition_range(&self.seats, 0..8);

        Seat::new(row, seat)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Seat {
    pub row: u32,
    pub column: u32,
}

impl Seat {
    pub const fn new(row: u32, column: u32) -> Self {
        Seat { row, column }
    }

    pub const fn id(self) -> u32 {
        self.column + self.row * 8
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

impl FromStr for BoardingPass {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::ensure!(s.len() == 7 + 3);

        let (rows, seats) = s.split_at(7);

        let rows = rows
            .chars()
            .map(|c| match c {
                'F' => Ok(Direction::Down),
                'B' => Ok(Direction::Up),
                other => anyhow::bail!(
                    "Expected \"F\" or \"B\", found \"{}\"",
                    other
                ),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let seats = seats
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Down),
                'R' => Ok(Direction::Up),
                other => anyhow::bail!(
                    "Expected \"L\" or \"R\", found \"{}\"",
                    other
                ),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(BoardingPass { rows, seats })
    }
}

fn partition_range(commands: &[Direction], range: Range<u32>) -> u32 {
    let Range { start, end } = range;
    let midpoint = (start + end) / 2;

    if commands.is_empty() {
        return midpoint;
    }

    let (direction, rest) = commands.split_first().expect("already checked");

    match direction {
        Direction::Up => partition_range(rest, midpoint..end),
        Direction::Down => partition_range(rest, start..midpoint),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_seat_id_calcs() {
        let inputs = vec![
            (Seat::new(70, 7), 567),
            (Seat::new(14, 7), 119),
            (Seat::new(102, 4), 820),
        ];

        for (seat, should_be) in inputs {
            let got = seat.id();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn find_location_for_known_boarding_passes() {
        let inputs = vec![
            ("BFFFBBFRRR", Seat::new(70, 7)),
            ("FFFBBBFRRR", Seat::new(14, 7)),
            ("BBFFBBFRLL", Seat::new(102, 4)),
        ];

        for (boarding_pass, should_be) in inputs {
            let boarding_pass: BoardingPass = boarding_pass.parse().unwrap();
            let got = boarding_pass.location();

            assert_eq!(got, should_be);
        }
    }
}
