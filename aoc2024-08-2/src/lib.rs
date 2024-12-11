use common::parse::{self};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter,
    ops::{Add, AddAssign, Sub},
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

impl AddAssign for Position {
    fn add_assign(&mut self, Self { x, y }: Self) {
        self.x += x;
        self.y += y;
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
    fn antinodes(&self, bounds: Bounds) -> impl Iterator<Item = Position> + '_ {
        self.0.iter().tuple_combinations().flat_map(move |(a, b)| {
            let fwd_delta = a - b;
            let rev_delta = b - a;
            let mut fwd_pos = a - &fwd_delta;
            let mut rev_pos = b - &rev_delta;
            let fwd = iter::from_fn(move || {
                fwd_pos += fwd_delta;
                if bounds.contains(&fwd_pos) {
                    Some(fwd_pos)
                } else {
                    None
                }
            });
            let rev = iter::from_fn(move || {
                rev_pos += rev_delta;
                if bounds.contains(&rev_pos) {
                    Some(rev_pos)
                } else {
                    None
                }
            });

            itertools::interleave(fwd, rev)
        })
    }
}

#[derive(Clone, Copy)]
struct Bounds {
    x: i64,
    y: i64,
}

impl Bounds {
    fn contains(&self, Position { x, y }: &Position) -> bool {
        x < &self.x && y < &self.y && *x >= 0 && *y >= 0
    }
}

impl Map {
    fn antinodes(&self) -> impl Iterator<Item = Position> + '_ {
        self.antenae
            .values()
            .flat_map(|nodes| Antenae::antinodes(nodes, self.bounds()))
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

    fn bounds(&self) -> Bounds {
        Bounds {
            x: self.width,
            y: self.height,
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
        assert_eq!(num_antinodes(example.lines().map(String::from))?, 34);
        Ok(())
    }
}
