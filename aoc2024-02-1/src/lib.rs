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

#[derive(PartialEq, Eq)]
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

fn until_err<T>(err: &mut &mut Result<()>, item: Result<T>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
    }
}

impl Levels {
    fn parse(input: &str) -> parse::Result<Self> {
        use nom::character::complete::u64;
        let (_, levels) = separated_list1(space1, u64)(input)?;
        Ok(Self(levels))
    }

    fn safe(&self) -> bool {
        let mut bounded: Result<()> = Ok(());
        let monotonic = self
            .into_iter()
            .tuple_windows()
            .map(|(prev, next)| match *next as i64 - *prev as i64 {
                0 => Err(Error::ConsecutiveEquals),
                1..=3 => Ok(Direction::Increasing),
                -3..=-1 => Ok(Direction::Decreasing),
                _ => Err(Error::BigJump),
            })
            .scan(&mut bounded, until_err)
            .all_equal();
        bounded.is_ok() && monotonic
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
        assert_eq!(num_safe_reports(example.lines().map(String::from))?, 2);
        Ok(())
    }
}
