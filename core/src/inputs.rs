use std::{convert::TryFrom, ops::Deref, str::FromStr};

/// A specialised input for handling lists of items, where each item is on its
/// own line.
pub struct Lines<T>(pub Vec<T>);

impl<T> FromStr for Lines<T>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Lines::try_from(s)
    }
}

impl<'input, T> TryFrom<&'input str> for Lines<T>
where
    T: FromStr,
{
    type Error = T::Err;

    fn try_from(s: &'input str) -> Result<Self, Self::Error> {
        let mut items = Vec::new();

        for line in s.lines() {
            if !line.is_empty() {
                let item = line.trim().parse()?;
                items.push(item);
            }
        }

        Ok(Lines(items))
    }
}

impl<T> Deref for Lines<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> IntoIterator for Lines<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct GroupedLines<'input>(std::str::Lines<'input>);

impl<'input> TryFrom<&'input str> for GroupedLines<'input> {
    type Error = std::convert::Infallible;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        Ok(GroupedLines(value.lines()))
    }
}

impl<'input> Iterator for GroupedLines<'input> {
    type Item = Vec<&'input str>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::new();

        while let Some(line) = self.0.next() {
            if group.is_empty() && line.is_empty() {
                continue;
            } else if line.is_empty() {
                break;
            } else {
                group.push(line);
            }
        }

        if group.is_empty() {
            None
        } else {
            Some(group)
        }
    }
}
