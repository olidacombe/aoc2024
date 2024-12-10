use common::parse::{self};
use rayon::prelude::*;
use std::{collections::HashSet, fmt::Display, ops::Add};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error("missing start position ('^')")]
    MissingStartPosition,
}

pub fn num_loopy_obstruction_positions(it: impl Iterator<Item = String>) -> Result<usize> {
    let mut world = Map::default();
    let mut start_pos = None;
    for row in it {
        start_pos = start_pos.or(world.push_row(&row)); // there _has_ to be a nicer pattern for this!
    }
    let Some(position) = start_pos else {
        return Err(Error::MissingStartPosition);
    };

    println!("{} obstacles", world.obstacles.len());
    println!("{}x{} grid", world.width, world.height);
    println!(
        "{} possible obstacle locations",
        world.width * world.height - world.obstacles.len() as i64 - 1
    );

    let maps = world.s_plus_one_obstacle(&position);
    println!("{} candidate maps", maps.len());
    let guards = maps.par_iter().map(|map| Guard::new(map, position));
    // guards.for_each(|guard| assert!(guard.map.obstacles.len() == 811));

    Ok(guards
        .filter_map(|mut guard| if guard.loops() { Some(()) } else { None })
        .count())
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct PosMentum {
    position: Position,
    direction: Direction,
}

enum GuardState {
    Patrolling,
    Exited,
    Looping,
}

struct Guard<'a> {
    seen_states: HashSet<PosMentum>,
    posmentum: PosMentum,
    map: &'a Map,
}

impl<'a> Guard<'a> {
    fn new(map: &'a Map, position: Position) -> Self {
        let posmentum = PosMentum {
            position,
            direction: Direction::default(),
        };
        Self {
            seen_states: HashSet::from([posmentum]),
            posmentum,
            map,
        }
    }

    fn loops(&mut self) -> bool {
        loop {
            match self.step() {
                GuardState::Exited => {
                    return false;
                }
                GuardState::Looping => {
                    return true;
                }
                _ => continue,
            }
        }
    }

    fn step(&mut self) -> GuardState {
        {
            let PosMentum {
                ref mut position,
                ref mut direction,
            } = self.posmentum;
            let next = *position + direction;
            if !self.map.covers(&next) {
                return GuardState::Exited;
            }
            if self.map.has_obstacle_at(&next) {
                direction.turn_right();
                *position = *position + direction;
            } else {
                *position = next;
            }
        }
        if !self.seen_states.insert(self.posmentum) {
            // true when already seen
            return GuardState::Looping;
        }
        GuardState::Patrolling
    }
}

#[derive(Default, Clone)]
struct Map {
    obstacles: HashSet<Position>,
    width: i64,
    height: i64,
}

impl Add<Position> for &Map {
    type Output = Map;
    fn add(self, rhs: Position) -> Self::Output {
        let mut ret = self.clone();
        ret.obstacles.insert(rhs);
        ret
    }
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

    fn s_plus_one_obstacle(&self, start_pos: &Position) -> Vec<Self> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Position { x, y }))
            .filter(|position| position != start_pos && !self.has_obstacle_at(position))
            .map(|position| {
                // println!("{position}");
                self + position
            })
            .collect()
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
            num_loopy_obstruction_positions(example.lines().map(String::from))?,
            6
        );
        Ok(())
    }

    #[test]
    fn full_example_plus() -> Result<()> {
        let example = indoc! {"
            ............
            .....#......
            ..........#.
            ............
            ...#........
            ........#...
            ............
            ..#..^......
            .........#..
            .#..........
            .......#....
            ............
        "};
        assert_eq!(
            num_loopy_obstruction_positions(example.lines().map(String::from))?,
            6
        );
        Ok(())
    }
}
