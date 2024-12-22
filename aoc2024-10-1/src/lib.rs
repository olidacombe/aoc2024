use std::collections::HashSet;

use common::parse::{self};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn sum_trailhead_scores(it: impl Iterator<Item = String>) -> Result<usize> {
    let map: Map = it.into();
    let bases = map.basecamps();
    Ok(bases
        .iter()
        .map(|base| map.peaks_reachable_from(base).len())
        .sum())
}

struct Row(Vec<u64>);

impl IntoIterator for Row {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Row {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self(
            value
                .chars()
                .filter_map(|ch| ch.to_digit(10).map(|i| i as u64))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
}

struct Map(Vec<Row>);

impl Map {
    fn basecamps(&self) -> HashSet<Coordinate> {
        let mut basecamps = HashSet::new();
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.into_iter().enumerate() {
                if *col == 0 {
                    basecamps.insert(Coordinate {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }
        basecamps
    }

    fn get_elevation(&self, coord: &Coordinate) -> Option<u64> {
        let Coordinate { x, y } = coord;
        let Ok::<usize, _>(x) = (*x).try_into() else {
            return None;
        };
        let Ok::<usize, _>(y) = (*y).try_into() else {
            return None;
        };
        self.0.get(y).and_then(|row| row.0.get(x).copied())
    }

    fn neighbours(&self, coord: &Coordinate) -> Vec<Coordinate> {
        let mut ret = Vec::new();
        let Some(elevation) = self.get_elevation(coord) else {
            return ret;
        };
        if elevation == 9 {
            return ret;
        }
        let Coordinate { x, y } = coord;
        let adjacents = [
            Coordinate { x: x - 1, y: *y },
            Coordinate { x: x + 1, y: *y },
            Coordinate { x: *x, y: y - 1 },
            Coordinate { x: *x, y: y + 1 },
        ];
        for neighbour in adjacents {
            if let Some(neighbour_elevation) = self.get_elevation(&neighbour) {
                if neighbour_elevation == elevation + 1 {
                    ret.push(neighbour);
                }
            }
        }
        ret
    }

    fn peaks_reachable_from(&self, coord: &Coordinate) -> HashSet<Coordinate> {
        let Some(elevation) = self.get_elevation(coord) else {
            return HashSet::new();
        };
        if elevation == 9 {
            HashSet::from([*coord])
        } else {
            self.neighbours(coord)
                .iter()
                .fold(HashSet::new(), |acc, coord| {
                    &acc | &self.peaks_reachable_from(coord)
                })
        }
    }
}

impl<I, S> From<I> for Map
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn from(lines: I) -> Self {
        Self(lines.map(|line| Row::from(line.as_ref())).collect())
    }
}

impl IntoIterator for Map {
    type Item = Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = &'a Row;
    type IntoIter = std::slice::Iter<'a, Row>;
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
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "};
        assert_eq!(sum_trailhead_scores(example.lines().map(String::from))?, 36);
        Ok(())
    }
}
