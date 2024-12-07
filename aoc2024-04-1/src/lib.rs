use std::ops::AddAssign;

use common::parse::{self};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn num_xmas_hits(it: impl Iterator<Item = String>) -> Result<usize> {
    let grid = Grid::from(it);
    let xs = grid.find('X');
    let mut searchers: Vec<_> = xs
        .into_iter()
        .flat_map(|coordinate| grid.searchers(&coordinate))
        .collect();
    Ok(searchers
        .iter_mut()
        .filter_map(|searcher| if searcher.hit() { Some(()) } else { None })
        .count())
}

struct Row(Vec<char>);

impl From<String> for Row {
    fn from(value: String) -> Self {
        Self(value.chars().collect())
    }
}

impl IntoIterator for Row {
    type Item = char;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Row {
    type Item = &'a char;
    type IntoIter = std::slice::Iter<'a, char>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Default)]
enum Xmas {
    #[default]
    X,
    M,
    A,
    S,
}

impl Xmas {
    fn next(&mut self) -> bool {
        match self {
            Self::X => {
                *self = Self::M;
                true
            }
            Self::M => {
                *self = Self::A;
                true
            }
            Self::A => {
                *self = Self::S;
                true
            }
            Self::S => false,
        }
    }
}

impl PartialEq<char> for Xmas {
    fn eq(&self, other: &char) -> bool {
        match self {
            Self::X => *other == 'x',
            Self::M => *other == 'm',
            Self::A => *other == 'a',
            Self::S => *other == 's',
        }
    }
}

enum Direction {
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
}

struct Searcher<'a> {
    grid: &'a Grid,
    position: Coordinate,
    xmas: Xmas,
    direction: Direction,
}

impl<'a> Searcher<'a> {
    fn hit(&'a mut self) -> bool {
        while self.xmas.next() {
            self.position += &self.direction;
            if self.xmas != self.grid.char(&self.position) {
                return false;
            }
        }
        true
    }
}

struct Grid(Vec<Row>);

impl Grid {
    fn find(&self, char: char) -> Vec<Coordinate> {
        self.into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter().enumerate().filter_map(move |(x, ch)| {
                    if *ch == char {
                        Some(Coordinate {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn char(&self, coordinate: &Coordinate) -> char {
        self.0[coordinate.y as usize].0[coordinate.x as usize]
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].0.len()
    }

    fn searchers(&self, Coordinate { x, y }: &Coordinate) -> Vec<Searcher> {
        let mut directions = Vec::new();
        if *x > 2 {
            directions.push(Direction::W);
            if *y > 2 {
                directions.push(Direction::NW);
                directions.push(Direction::N);
            }
            if *y < self.height() as i64 - 3 {
                directions.push(Direction::S);
                directions.push(Direction::SW);
            }
        }
        if *x < self.width() as i64 - 3 {
            directions.push(Direction::E);
            if *y > 2 {
                directions.push(Direction::NE);
                directions.push(Direction::N);
            }
            if *y < self.height() as i64 - 3 {
                directions.push(Direction::SE);
                directions.push(Direction::S);
            }
        }
        directions
            .into_iter()
            .map(|direction| Searcher {
                grid: self,
                direction,
                xmas: Xmas::default(),
                position: Coordinate { x: *x, y: *y },
            })
            .collect()
    }
}

impl IntoIterator for Grid {
    type Item = Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Row;
    type IntoIter = std::slice::Iter<'a, Row>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<I> From<I> for Grid
where
    I: Iterator<Item = String>,
{
    fn from(value: I) -> Self {
        Self(value.map(Row::from).collect())
    }
}

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl AddAssign<&Direction> for Coordinate {
    fn add_assign(&mut self, rhs: &Direction) {
        match rhs {
            Direction::E => {
                self.x += 1;
            }
            Direction::NE => {
                self.y -= 1;
                self.x += 1;
            }
            Direction::N => {
                self.y -= 1;
            }
            Direction::NW => {
                self.y -= 1;
                self.x -= 1;
            }
            Direction::W => {
                self.x -= 1;
            }
            Direction::SW => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::S => {
                self.y += 1;
            }
            Direction::SE => {
                self.x += 1;
                self.y += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "};
        assert_eq!(num_xmas_hits(example.lines().map(String::from))?, 18);
        Ok(())
    }
}
