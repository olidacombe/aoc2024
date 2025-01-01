use std::collections::VecDeque;

use common::parse::{self, Parse};
use ilog::IntLog;
use nom::{character::streaming::space1, multi::separated_list1};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn num_stones(input: &str, count: usize) -> Result<usize> {
    let (_, stones) = Stones::parse(input).map_err(|e| Error::Parse(e.into()))?;
    Ok(stones.num_descendents_after(count))
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct Stone(u64);

type Cache = std::collections::HashMap<(Stone, usize), usize>;

impl Stone {
    fn num_descendents_after(self, n: usize, cache: &mut Cache) -> usize {
        if let Some(answer) = cache.get(&(self.clone(), n)) {
            return *answer;
        }
        if n == 0 {
            return 1;
        }
        let ret = if self.0 == 0 {
            Stone(1).num_descendents_after(n - 1, cache)
        } else if let Some((left, right)) = self.split() {
            left.num_descendents_after(n - 1, cache) + right.num_descendents_after(n - 1, cache)
        } else {
            Stone(self.0 * 2024).num_descendents_after(n - 1, cache)
        };
        cache.insert((self, n), ret);
        ret
    }

    fn split(&self) -> Option<(Stone, Stone)> {
        let num_digits = self.0.log10() + 1;
        if num_digits % 2 == 1 {
            return None;
        }
        static BASE: u64 = 10;
        let order = BASE.pow(num_digits as u32 / 2);
        let right = Self(self.0 % order);
        let left = Self(self.0 / order);
        Some((left, right))
    }
}

#[derive(Debug)]
struct Stones(VecDeque<Stone>);

impl Stones {
    fn num_descendents_after(self, n: usize) -> usize {
        let mut cache = Cache::default();
        self.0
            .into_iter()
            .map(|stone| stone.num_descendents_after(n, &mut cache))
            .sum()
    }
}

impl Parse for Stones {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::u64;
        let (rest, stones) = separated_list1(space1, u64)(input)?;
        let stones = stones.into_iter().map(Stone).collect();
        Ok((rest, Self(stones)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            125 17
        "};
        assert_eq!(num_stones(example, 25)?, 55312);
        Ok(())
    }
}
