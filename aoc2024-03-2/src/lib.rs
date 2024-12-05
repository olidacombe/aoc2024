use common::parse::{self, Parse};
use nom::{
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn multiplication_sum(input: &str) -> Result<u64> {
    let active_sections = ActiveSections::parse(input);
    let active_muls = active_sections.into_iter().flat_map(Muls::parse);
    Ok(active_muls.into_iter().map(|mul| mul.tiplied()).sum())
}

enum Section<'a> {
    Active(&'a str),
    Inactive(&'a str),
}

impl Section<'_> {
    fn next(self) -> (Self, Option<Self>) {
        match self {
            Self::Active(split_me) => {
                if let Some(idx) = split_me.find("don't()") {
                    let (prev, next) = split_me.split_at(idx);
                    (Self::Active(prev), Some(Self::Inactive(next)))
                } else {
                    (Self::Active(split_me), None)
                }
            }
            Self::Inactive(split_me) => {
                if let Some(idx) = split_me.find("do()") {
                    let (prev, next) = split_me.split_at(idx);
                    (Self::Inactive(prev), Some(Self::Active(next)))
                } else {
                    (Self::Inactive(split_me), None)
                }
            }
        }
    }
}

struct ActiveSections<'a>(Vec<&'a str>);

impl<'a> IntoIterator for ActiveSections<'a> {
    type Item = &'a str;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ActiveSections<'a> {
    fn parse(input: &'a str) -> Self {
        let mut section = Section::Active(input);
        let mut sections = Vec::new();
        loop {
            match section.next() {
                (prev, Some(next)) => {
                    if let Section::Active(segment) = prev {
                        sections.push(segment);
                    }
                    section = next;
                }
                (prev, None) => {
                    if let Section::Active(segment) = prev {
                        sections.push(segment);
                    }
                    break;
                }
            }
        }
        Self(sections)
    }
}

struct Mul {
    x: u64,
    y: u64,
}

impl Mul {
    fn tiplied(&self) -> u64 {
        self.x * self.y
    }
}

impl Parse for Mul {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::char;
        use nom::character::complete::u64;
        let (rest, (x, y)) =
            delimited(tag("mul("), separated_pair(u64, char(','), u64), char(')'))(input)?;
        Ok((rest, Mul { x, y }))
    }
}

struct Muls(Vec<Mul>);

impl Muls {
    fn parse(input: &str) -> Self {
        Self(
            input
                .match_indices("mul(")
                .filter_map(|(idx, _)| Mul::parse(&input[idx..]).map(|(_, mul)| mul).ok())
                .collect(),
        )
    }
}

impl IntoIterator for Muls {
    type Item = Mul;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Muls {
    type Item = &'a Mul;
    type IntoIter = std::slice::Iter<'a, Mul>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "};
        assert_eq!(multiplication_sum(example)?, 161);
        Ok(())
    }
}
