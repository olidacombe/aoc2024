use std::fmt::Display;

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
    let candidate_as = grid.find('A');
    let mut searchers: Vec<_> = candidate_as
        .into_iter()
        .filter_map(|coordinate| grid.searcher(&coordinate))
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

struct Searcher<'a> {
    grid: &'a Grid,
    position: Coordinate,
}

impl Display for Searcher<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for y in self.position.y - 1..=self.position.y + 1 {
            for x in self.position.x - 1..=self.position.x + 1 {
                out.push(self.grid.char(&Coordinate { x, y }));
            }
            out.push('\n');
        }
        write!(f, "{out}")
    }
}

impl<'a> Searcher<'a> {
    fn hit(&'a mut self) -> bool {
        let Coordinate { x, y } = self.position;
        let diagonals = [
            [
                Coordinate { x: x - 1, y: y - 1 },
                Coordinate { x: x + 1, y: y + 1 },
            ],
            [
                Coordinate { x: x + 1, y: y - 1 },
                Coordinate { x: x - 1, y: y + 1 },
            ],
        ];
        for diagonal in diagonals {
            match self.grid.char(&diagonal[0]) {
                'M' => {
                    if self.grid.char(&diagonal[1]) != 'S' {
                        return false;
                    }
                }
                'S' => {
                    if self.grid.char(&diagonal[1]) != 'M' {
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

    fn searcher(&self, Coordinate { x, y }: &Coordinate) -> Option<Searcher> {
        if *x > 0 && *x < self.width() as i64 - 1 && *y > 0 && *y < self.height() as i64 - 1 {
            Some(Searcher {
                grid: self,
                position: Coordinate { x: *x, y: *y },
            })
        } else {
            None
        }
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn eg_1() -> Result<()> {
        let example = indoc! {"
            M.M
            .A.
            S.S
        "};
        assert_eq!(num_xmas_hits(example.lines().map(String::from))?, 1);
        Ok(())
    }

    #[test]
    fn eg_2() -> Result<()> {
        let example = indoc! {"
            M.S
            .A.
            M.S
        "};
        assert_eq!(num_xmas_hits(example.lines().map(String::from))?, 1);
        Ok(())
    }

    #[test]
    fn eg_3() -> Result<()> {
        let example = indoc! {"
            S.S
            .A.
            M.M
        "};
        assert_eq!(num_xmas_hits(example.lines().map(String::from))?, 1);
        Ok(())
    }

    #[test]
    fn eg_4() -> Result<()> {
        let example = indoc! {"
            S.M
            .A.
            S.M
        "};
        assert_eq!(num_xmas_hits(example.lines().map(String::from))?, 1);
        Ok(())
    }

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
        assert_eq!(num_xmas_hits(example.lines().map(String::from))?, 9);
        Ok(())
    }
}
