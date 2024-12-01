use std::ops::Sub;

use common::parse::{self};
use nom::{character::complete::space1, sequence::separated_pair};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn l2_distance_between_lists(it: impl Iterator<Item = String>) -> Result<u64> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = it
        .filter_map(|row| {
            UnsortedRow::parse(row.as_str())
                .ok()
                .map(|UnsortedRow { left, right }| (left, right))
        })
        .unzip();
    left.sort();
    right.sort();
    Ok(List(left) - List(right))
}

struct UnsortedRow {
    left: u64,
    right: u64,
}

struct List(Vec<u64>);

impl IntoIterator for List {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Sub for List {
    type Output = u64;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0
            .iter()
            .zip(rhs)
            .map(|(left, right)| left.abs_diff(right))
            .sum()
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
        assert_eq!(
            l2_distance_between_lists(example.lines().map(String::from))?,
            11
        );
        Ok(())
    }
}
