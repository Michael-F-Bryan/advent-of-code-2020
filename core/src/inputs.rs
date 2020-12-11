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
