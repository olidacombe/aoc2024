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
        let mut tester = LevelsTester::new(&self.0);
        tester.any(|test| test)
    }
}

#[derive(Debug)]
struct LevelsTester<'a> {
    levels: &'a [u64],
    skip_index: usize,
}

impl<'a> LevelsTester<'a> {
    fn new(levels: &'a [u64]) -> Self {
        Self {
            levels,
            skip_index: 0,
        }
    }

    fn test(&self) -> bool {
        let it = self.levels.iter().enumerate().filter_map(|(idx, level)| {
            if idx != self.skip_index {
                Some(level)
            } else {
                None
            }
        });
        let direction = OnceCell::<Direction>::new();
        for (prev, next) in it.tuple_windows() {
            match *next as i64 - *prev as i64 {
                1..=3 => {
                    if *direction.get_or_init(|| Direction::Increasing) != Direction::Increasing {
                        return false;
                    }
                }
                -3..=-1 => {
                    if *direction.get_or_init(|| Direction::Decreasing) != Direction::Decreasing {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        true
    }
}

impl Iterator for LevelsTester<'_> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_index >= self.levels.len() {
            return None;
        }
        let ret = self.test();
        self.skip_index += 1;
        Some(ret)
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
