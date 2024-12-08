use std::{collections::HashSet, ops::Add};

use common::parse::{self};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error("missing start position ('^')")]
    MissingStartPosition,
}

pub fn num_distinct_guard_positions(it: impl Iterator<Item = String>) -> Result<usize> {
    let mut world = Map::default();
    let mut start_pos = None;
    for row in it {
        start_pos = start_pos.or(world.push_row(&row)); // there _has_ to be a nicer pattern for this!
    }
    let Some(position) = start_pos else {
        return Err(Error::MissingStartPosition);
    };

    let mut guard = Guard {
        position,
        direction: Direction::default(),
        map: &world,
    };

    let mut visited = HashSet::from([position]);

    while let Some(position) = guard.step() {
        visited.insert(position);
    }

    Ok(visited.len())
}

#[derive(Default)]
enum Direction {
    E,
    #[default]
    N,
    W,
    S,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            Direction::N => Direction::E,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Add<&Direction> for Position {
    type Output = Self;
    fn add(mut self, rhs: &Direction) -> Self::Output {
        match rhs {
            Direction::E => {
                self.x += 1;
            }
            Direction::N => {
                self.y -= 1;
            }
            Direction::W => {
                self.x -= 1;
            }
            Direction::S => {
                self.y += 1;
            }
        }
        self
    }
}

struct Guard<'a> {
    position: Position,
    direction: Direction,
    map: &'a Map,
}

impl Guard<'_> {
    fn step(&mut self) -> Option<Position> {
        let next = self.position + &self.direction;
        if self.map.has_obstacle_at(&next) {
            self.direction.turn_right();
            self.position = self.position + &self.direction;
        } else {
            self.position = next;
        }
        if self.map.covers(&self.position) {
            return Some(self.position);
        }
        None
    }
}

#[derive(Default)]
struct Map {
    obstacles: HashSet<Position>,
    width: i64,
    height: i64,
}

impl Map {
    fn push_row(&mut self, row: &str) -> Option<Position> {
        self.obstacles
            .extend(row.match_indices('#').map(|(idx, _)| Position {
                x: idx as i64,
                y: self.height,
            }));
        self.width = row.len() as i64;
        self.height += 1;
        row.find('^').map(|x| Position {
            x: x as i64,
            y: self.height - 1,
        })
    }

    fn covers(&self, Position { x, y }: &Position) -> bool {
        x < &self.width && y < &self.height && *x >= 0 && *y >= 0
    }

    fn has_obstacle_at(&self, position: &Position) -> bool {
        self.obstacles.contains(position)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "};
        assert_eq!(
            num_distinct_guard_positions(example.lines().map(String::from))?,
            41
        );
        Ok(())
    }
}
