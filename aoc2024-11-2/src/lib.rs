use std::collections::VecDeque;

use cache_macro::cache;
use common::parse::{self, Parse};
use ilog::IntLog;
use lru_cache::LruCache;
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

#[cache(LruCache : LruCache::new(200000))]
fn num_descendents_after(stone: Stone, n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    stone
        .mutate()
        .0
        .iter()
        .map(|stone| stone.num_descendents_after(n - 1))
        .sum()
}

impl Stone {
    fn num_descendents_after(&self, n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        self.clone()
            .mutate()
            .0
            .iter()
            .map(|stone| stone.num_descendents_after(n - 1))
            .sum()
    }

    fn mutate(self) -> Stones {
        if self.0 == 0 {
            return Stones(VecDeque::from([Stone(1)]));
        }
        if let Some((left, right)) = self.split() {
            return Stones(VecDeque::from([left, right]));
        }
        Stones(VecDeque::from([Stone(self.0 * 2024)]))
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
    fn blink(&mut self) {
        let mut new = VecDeque::new();
        while let Some(stone) = self.0.pop_front() {
            new.append(&mut stone.mutate().0);
        }
        self.0 = new;
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn num_descendents_after(&self, n: usize) -> usize {
        self.0
            .iter()
            .map(|stone| stone.num_descendents_after(n))
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
