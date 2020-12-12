use anyhow::Error;
use aoc_core::GroupedLines;
use bitvec::{slice::BitSlice, vec::BitVec};
use std::{convert::TryFrom, str::FromStr};

/// Day 6a: Custom Customs (part a)
///
/// # Description
///
/// As your flight approaches the regional airport where you'll switch to a much
/// larger plane, customs declaration forms are distributed to the passengers.
///
/// The form asks a series of 26 yes-or-no questions marked a through z. All you
/// need to do is identify the questions for which anyone in your group answers
/// "yes". Since your group is just you, this doesn't take very long.
///
/// However, the person sitting next to you seems to be experiencing a language
/// barrier and asks if you can help. For each of the people in their group, you
/// write down the questions for which they answer "yes", one per line. For
/// example:
///
/// ```text
/// abcx
/// abcy
/// abcz
/// ```
///
/// In this group, there are 6 questions to which anyone answered "yes": a, b, c,
/// x, y, and z. (Duplicate answers to the same question don't count extra; each
/// question counts at most once.)
///
/// Another group asks for your help, then another, and eventually you've
/// collected answers from every group on the plane (your puzzle input). Each
/// group's answers are separated by a blank line, and within each group, each
/// person's answers are on a single line. For example:
///
/// ```text
/// abc
///
/// a
/// b
/// c
///
/// ab
/// ac
///
/// a
/// a
/// a
/// a
///
/// b
/// ```
///
/// This list represents answers from five groups:
///
/// ```text
/// The first group contains one person who answered "yes" to 3 questions: a, b, and c.
/// The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
/// The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
/// The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
/// The last group contains one person who answered "yes" to only 1 question, b.
/// ```
///
/// In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
///
/// For each group, count the number of questions to which anyone answered "yes".
/// What is the sum of those counts?
#[aoc_macros::challenge]
pub fn part_1(responses: Responses) -> Result<usize, Error> {
    Ok(responses
        .0
        .iter()
        .map(|group| group.merge_any())
        .map(|merged_answer| merged_answer.pop_count())
        .sum())
}

/// Day 6b: ass
///
/// # Description
///
/// As you finish the last group's customs declaration, you notice that you
/// misread one word in the instructions:
///
/// You don't need to identify the questions to which anyone answered "yes"; you
/// need to identify the questions to which everyone answered "yes"!
///
/// Using the same example as above:
///
/// ```text
/// abc
///
/// a
/// b
/// c
///
/// ab
/// ac
///
/// a
/// a
/// a
/// a
///
/// b
/// ```
///
/// This list represents answers from five groups:
///
/// ```text
/// In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
/// In the second group, there is no question to which everyone answered "yes".
/// In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
/// In the fourth group, everyone answered yes to only 1 question, a.
/// In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
/// ```
///
/// In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
///
/// For each group, count the number of questions to which everyone answered
/// "yes". What is the sum of those counts?
#[aoc_macros::challenge]
pub fn part_2(responses: Responses) -> Result<usize, Error> {
    Ok(responses
        .0
        .iter()
        .map(|group| group.merge_all())
        .map(|merged_answer| merged_answer.pop_count())
        .sum())
}

#[derive(Debug, Clone, PartialEq)]
pub struct Responses(Vec<ResponseGroup>);

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseGroup(Vec<Response>);

impl ResponseGroup {
    pub fn merge_any(&self) -> Response {
        self.merge_with(Response::default(), |acc, elem| {
            *acc |= elem.iter().copied();
        })
    }

    pub fn merge_all(&self) -> Response {
        self.merge_with(Response(BitVec::repeat(true, 26)), |acc, elem| {
            *acc &= elem.iter().copied();
        })
    }

    fn merge_with<F>(&self, init: Response, mut merger: F) -> Response
    where
        F: FnMut(&mut BitVec, &BitSlice),
    {
        self.0.iter().fold(init, |mut acc, elem| {
            if elem.0.len() > acc.0.len() {
                acc.0.resize(elem.0.len(), false);
            }

            merger(&mut acc.0, &elem.0);

            acc
        })
    }
}

impl<'input> TryFrom<&'input str> for Responses {
    type Error = Error;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let groups = GroupedLines::try_from(value)?;
        Responses::try_from(groups)
    }
}

impl<'input> TryFrom<GroupedLines<'input>> for Responses {
    type Error = Error;

    fn try_from(value: GroupedLines<'input>) -> Result<Self, Self::Error> {
        let mut response_groups = Vec::new();

        for lines in value {
            let mut group = Vec::with_capacity(lines.len());

            for line in lines {
                let response: Response = line.parse()?;
                group.push(response);
            }

            response_groups.push(ResponseGroup(group));
        }

        Ok(Responses(response_groups))
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Response(BitVec);

impl Response {
    pub fn pop_count(&self) -> usize {
        self.0.iter().filter(|value| **value).count()
    }
}

impl<'input> TryFrom<&'input str> for Response {
    type Error = Error;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let mut answers = BitVec::repeat(false, 26);

        for letter in value.chars() {
            match letter {
                'a'..='z' => answers.set(letter as usize - 'a' as usize, true),
                _ => todo!(),
            }
        }

        Ok(Response(answers))
    }
}

impl FromStr for Response {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Response::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_responses() {
        let inputs = vec![
            ("abcx", &[0, 1, 2, 23]),
            ("abcy", &[0, 1, 2, 24]),
            ("abcz", &[0, 1, 2, 25]),
        ];

        for (raw, set_indices) in inputs {
            let got: Response = raw.parse().unwrap();

            for index in 0..26 {
                let should_be = set_indices.contains(&index);
                assert_eq!(got.0[index as usize], should_be);
            }
        }
    }
}
