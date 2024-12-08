use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use common::parse::{self, Parse};
use nom::{multi::separated_list1, sequence::separated_pair};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn sum_middle_fixed_updates(mut it: impl Iterator<Item = String>) -> Result<u64> {
    let rules: Rules = it
        .by_ref()
        .map_while(|line| {
            Precedence::parse(&line)
                .ok()
                .map(|(_, precedence)| precedence)
        })
        .into();
    let _ = it.by_ref().skip_while(|line| line == "\n");
    let invalid = it
        .filter_map(|ref line| Pages::parse(line).ok().map(|(_, pages)| pages))
        .filter(|pages| !rules.validate(pages));
    Ok(invalid.map(|pages| rules.sort(pages).middle()).sum())
}

struct Pages(Vec<u64>);

impl Pages {
    fn middle(&self) -> u64 {
        self.0[self.0.len() / 2]
    }
}

impl IntoIterator for Pages {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Pages {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Parse for Pages {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::{char, u64};
        let (rest, pages) = separated_list1(char(','), u64)(input)?;
        Ok((rest, Self(pages)))
    }
}

struct Precedence {
    predecessor: u64,
    successor: u64,
}

#[derive(Debug)]
struct Rules(HashMap<u64, HashSet<u64>>);

impl Rules {
    fn validate(&self, pages: &Pages) -> bool {
        let mut forbidden_successors = HashSet::<u64>::new();
        for page in pages {
            if forbidden_successors.contains(page) {
                return false;
            }
            if let Some(predecessors) = self.0.get(page) {
                forbidden_successors.extend(predecessors);
            }
        }
        true
    }

    fn sort(&self, mut pages: Pages) -> Pages {
        pages.0.sort_by(|a, b| {
            if let Some(predecessors) = self.0.get(b) {
                if predecessors.contains(a) {
                    return Ordering::Greater;
                }
            }
            if let Some(predecessors) = self.0.get(a) {
                if predecessors.contains(b) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
        pages
    }
}

impl<I> From<I> for Rules
where
    I: IntoIterator<Item = Precedence>,
{
    fn from(value: I) -> Self {
        let mut rules = HashMap::new();
        for rule in value.into_iter() {
            rules
                .entry(rule.successor)
                .and_modify(|predecessors: &mut HashSet<u64>| {
                    predecessors.insert(rule.predecessor);
                })
                .or_insert_with(|| HashSet::from([rule.predecessor]));
        }
        Self(rules)
    }
}

impl Parse for Precedence {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::char;
        use nom::character::complete::u64;
        let (rest, (predecessor, successor)) = separated_pair(u64, char('|'), u64)(input)?;
        Ok((
            rest,
            Self {
                predecessor,
                successor,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "};
        assert_eq!(
            sum_middle_fixed_updates(example.lines().map(String::from))?,
            123
        );
        Ok(())
    }
}
