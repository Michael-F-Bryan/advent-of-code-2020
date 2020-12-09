use std::{ops::Deref, str::FromStr};

/// A specialised input for handling lists of items, where each item is on its
/// own line.
pub struct Lines<T>(pub Vec<T>);

impl<T> FromStr for Lines<T>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
