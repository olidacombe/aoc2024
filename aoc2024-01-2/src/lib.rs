use std::collections::HashMap;

use common::parse::{self};
use nom::{character::complete::space1, sequence::separated_pair};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn similarity_score(it: impl Iterator<Item = String>) -> Result<u64> {
    let (left, right): (Vec<_>, Vec<_>) = it
        .filter_map(|row| {
            UnsortedRow::parse(row.as_str())
                .ok()
                .map(|UnsortedRow { left, right }| (left, right))
        })
        .unzip();
    let mut frequencies = Freq::default();
    for r in right {
        frequencies.push(r);
    }
    Ok(left.iter().map(|l| l * frequencies.get_count(*l)).sum())
}

struct UnsortedRow {
    left: u64,
    right: u64,
}

#[derive(Default)]
struct Freq(HashMap<u64, u64>);

impl Freq {
    fn get_count(&self, item: u64) -> u64 {
        self.0.get(&item).copied().unwrap_or_default()
    }

    fn push(&mut self, item: u64) {
        if let Some(count) = self.0.get_mut(&item) {
            *count += 1;
        } else {
            self.0.insert(item, 1);
        }
    }
}

struct List(Vec<u64>);

impl IntoIterator for List {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl UnsortedRow {
    fn parse(input: &str) -> parse::Result<Self> {
        use nom::character::complete::u64;
        let (_, (left, right)) = separated_pair(u64, space1, u64)(input)?;
        Ok(Self { left, right })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        assert_eq!(similarity_score(example.lines().map(String::from))?, 31);
        Ok(())
    }
}
