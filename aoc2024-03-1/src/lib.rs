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
    Ok(Muls::parse(input)
        .into_iter()
        .fold(0, |acc, mul| acc + mul.tiplied()))
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
