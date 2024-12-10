use common::parse::{self, Parse};
use ilog::IntLog;
use nom::{
    bytes::complete::tag, character::complete::space1, multi::separated_list1,
    sequence::separated_pair,
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn sum_achievable_test_values(it: impl Iterator<Item = String>) -> Result<u64> {
    Ok(it
        .filter_map(|ref line| {
            CalibrationEquation::parse(line)
                .ok()
                .map(|(_, equation)| equation)
        })
        .filter_map(|equation| {
            if equation.valid() {
                Some(equation.answer)
            } else {
                None
            }
        })
        .sum())
}

struct CalibrationEquation {
    answer: u64,
    variables: Variables,
}

impl CalibrationEquation {
    fn valid(&self) -> bool {
        find_answer(self.answer, &self.variables.0)
    }
}

impl Parse for CalibrationEquation {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::u64;
        let (rest, (answer, variables)) = separated_pair(u64, tag(": "), Variables::parse)(input)?;
        Ok((rest, Self { answer, variables }))
    }
}

struct Variables(Vec<u64>);

fn order(num: u64) -> u64 {
    static BASE: u64 = 10;
    BASE.pow(num.log10() as u32 + 1)
}

fn unconcat(lhs: u64, rhs: u64) -> u64 {
    lhs / order(rhs)
}

fn find_answer(answer: u64, variables: &[u64]) -> bool {
    let Some((last, rest)) = variables.split_last() else {
        return false;
    };
    if *last > answer {
        return false;
    }
    if rest.is_empty() {
        return *last == answer;
    }
    if find_answer(answer - last, rest) {
        return true;
    }
    if answer % last == 0 && find_answer(answer / last, rest) {
        return true;
    }
    if answer % order(*last) == *last {
        return find_answer(unconcat(answer, *last), rest);
    }
    false
}

impl IntoIterator for Variables {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Variables {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Parse for Variables {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::u64;
        let (rest, variables) = separated_list1(space1, u64)(input)?;
        Ok((rest, Self(variables)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "};
        assert_eq!(
            sum_achievable_test_values(example.lines().map(String::from))?,
            11387
        );
        Ok(())
    }

    #[test]
    fn simple_concat() -> Result<()> {
        let example = indoc! {"
            156: 15 6
        "};
        assert_eq!(
            sum_achievable_test_values(example.lines().map(String::from))?,
            156
        );
        Ok(())
    }

    #[test]
    fn test_order() {
        assert_eq!(order(123), 1000);
        // really we want it for this:
        assert_eq!(123456 % order(456), 456);
    }

    #[test]
    fn test_unconcat() {
        assert_eq!(unconcat(123456, 456), 123);
    }
}
