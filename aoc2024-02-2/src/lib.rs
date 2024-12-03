use std::cell::OnceCell;

use common::parse::{self};
use itertools::Itertools;
use nom::{character::complete::space1, multi::separated_list1};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error("consecutive levels are equal")]
    ConsecutiveEquals,
    #[error("jump > 3")]
    BigJump,
}

pub fn num_safe_reports(it: impl Iterator<Item = String>) -> Result<usize> {
    Ok(it
        .filter_map(|ref line| Levels::parse(line).ok())
        .filter(Levels::safe)
        .count())
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}

#[derive(Debug)]
struct Levels(Vec<u64>);

impl IntoIterator for Levels {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Levels {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Levels {
    fn parse(input: &str) -> parse::Result<Self> {
        use nom::character::complete::u64;
        let (_, levels) = separated_list1(space1, u64)(input)?;
        Ok(Self(levels))
    }

    fn safe(&self) -> bool {
        let tester = LevelsTester::new(&self.0);
        tester.test().into()
    }
}

enum LevelsTester<'a> {
    FirstRound(&'a [u64]),
    SecondRound(&'a [u64]),
    Failed,
}

impl<'a> LevelsTester<'a> {
    fn new(levels: &'a [u64]) -> Self {
        Self::FirstRound(levels)
    }

    fn failed(self, idx: usize) -> Self {
        // TODO
        // edge cases:
        // - if we fail at idx==0, then omitting 0th _or_ 1st element might fix it
        // so in this case, iterate backwards (add FirstRoundReverse variant)
        // - if we fail at penultimate idx, do the same?
        // (computationally inefficient but re-use efficient)
        if let Self::FirstRound(levels) = self {
            return Self::SecondRound(&levels[idx..]);
        }
        Self::Failed
    }

    fn test(self) -> Self {
        if let Self::FirstRound(levels) | Self::SecondRound(levels) = self {
            let mut it: Box<dyn Iterator<Item = (usize, &u64)>> =
                Box::new(levels.iter().enumerate());
            if let Self::SecondRound(_) = self {
                it = Box::new(it.filter(|(idx, _)| *idx != 1));
            }
            let direction = OnceCell::<Direction>::new();
            for ((idx, prev), (_, next)) in it.tuple_windows() {
                match *next as i64 - *prev as i64 {
                    1..=3 => {
                        if *direction.get_or_init(|| Direction::Increasing) != Direction::Increasing
                        {
                            return self.failed(idx);
                        }
                    }
                    -3..=-1 => {
                        if *direction.get_or_init(|| Direction::Decreasing) != Direction::Decreasing
                        {
                            return self.failed(idx);
                        }
                    }
                    _ => {
                        return self.failed(idx);
                    }
                }
            }
            self
        } else {
            self
        }
    }
}

impl From<LevelsTester<'_>> for bool {
    fn from(value: LevelsTester) -> Self {
        !matches!(value, LevelsTester::Failed)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        assert_eq!(num_safe_reports(example.lines().map(String::from))?, 4);
        Ok(())
    }
}
