use common::parse::{self};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter,
    ops::{Add, Sub},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn num_antinodes(it: impl Iterator<Item = String>) -> Result<usize> {
    let mut world = Map::default();
    for row in it {
        world.push_row(&row);
    }
    Ok(world.antinodes().count())
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Add for &Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Default)]
struct Map {
    antenae: HashMap<char, Antenae>,
    width: i64,
    height: i64,
}

struct Antenae(HashSet<Position>);

impl Antenae {
    fn antinodes(&self) -> impl Iterator<Item = Position> + '_ {
        self.0.iter().tuple_combinations().flat_map(|(a, b)| {
            let delta = a - b;
            iter::once(a + &delta).chain(iter::once(b - &delta))
        })
    }
}

impl Map {
    fn antinodes(&self) -> impl Iterator<Item = Position> + '_ {
        self.antenae
            .values()
            .flat_map(Antenae::antinodes)
            .filter(|position| self.contains(position))
            .unique()
    }

    fn push_row(&mut self, row: &str) {
        for (x, ch) in row.chars().enumerate().filter(|(_, ch)| *ch != '.') {
            let pos = Position {
                x: x as i64,
                y: self.height,
            };
            self.antenae
                .entry(ch)
                .and_modify(|locations| {
                    locations.0.insert(pos);
                })
                .or_insert_with(|| Antenae(HashSet::from([pos])));
        }
        self.width = row.len() as i64;
        self.height += 1;
    }

    fn contains(&self, Position { x, y }: &Position) -> bool {
        x < &self.width && y < &self.height && *x >= 0 && *y >= 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "};
        assert_eq!(num_antinodes(example.lines().map(String::from))?, 14);
        Ok(())
    }
}
