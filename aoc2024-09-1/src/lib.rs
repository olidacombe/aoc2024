use std::{collections::LinkedList, fmt::Display};

use common::parse::{self};
use itertools::Itertools;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn compacted_checksum(input: &str) -> Result<u32> {
    // make the "bold assumption" (same for the example and my test input)
    // that the input string has an odd number of chars (so no free space representation at the
    // end)
    let disk = Disk::from(input);
    println!("{disk}");
    Ok(disk.compacted_checksum())
}

#[derive(Debug)]
struct Blocks {
    file_id: Option<u32>,
    start_pos: u32,
    length: u32,
}

impl Blocks {
    fn position_sum(&self) -> u32 {
        let Self {
            length, start_pos, ..
        } = self;
        start_pos * length + (length * (length - 1)) / 2
    }

    fn is_file(&self) -> bool {
        self.file_id.is_some()
    }

    /// |-----------front-----------| |-back-|
    /// returns new_front, new_space, new_back
    fn backfill(mut self, mut rhs: Self) -> (Option<Self>, Option<Self>, Option<Self>) {
        if self.length >= rhs.length {
            let length = self.length - rhs.length;
            let start_pos = self.start_pos + rhs.length;
            self.length = rhs.length;
            self.file_id = rhs.file_id;
            (
                Some(self),
                if length > 0 {
                    Some(Blocks {
                        length,
                        start_pos,
                        file_id: None,
                    })
                } else {
                    None
                },
                None,
            )
        } else {
            self.file_id = rhs.file_id;
            rhs.length -= self.length; // we won't care about any data folling rhs
            (Some(self), None, Some(rhs))
        }
    }
}

#[derive(Debug)]
struct Disk(LinkedList<Blocks>);

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for block in &self.0 {
            for _ in 0..block.length {
                out += &block
                    .file_id
                    .map(|id| format!("{id}"))
                    .unwrap_or(".".to_string());
            }
        }
        write!(f, "{out}")
    }
}

impl Disk {
    fn pop_back_file(&mut self) -> Option<Blocks> {
        while let Some(blocks) = self.0.pop_back() {
            if blocks.is_file() {
                return Some(blocks);
            }
        }
        None
    }

    /// basically ruins this struct, but avoids awkward bouble-ended mutable iteration,
    /// also doesn't compact in any way ;)
    fn compacted_checksum(mut self) -> u32 {
        let mut checksum = 0;
        while let Some(blocks) = self.0.pop_front() {
            println!("{self} :: {checksum}");
            if let Some(file_id) = blocks.file_id {
                checksum += file_id * blocks.position_sum();
                continue;
            }
            let Some(back_file) = self.pop_back_file() else {
                break;
            };
            let (new_front, new_space, new_back) = blocks.backfill(back_file);
            if let Some(new_space) = new_space {
                self.0.push_front(new_space);
            }
            if let Some(new_front) = new_front {
                self.0.push_front(new_front);
            }
            if let Some(new_back) = new_back {
                self.0.push_back(new_back);
            }
        }
        checksum
    }
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut disk = LinkedList::new();
        let mut start_pos = 0;
        // add a "space" to the end so our tuples iterator doesn't stop early
        let hack = format!("{value}0");
        for (file_id, (length, space)) in
            hack.chars().map(|ch| ch.to_digit(10)).tuples().enumerate()
        {
            let Some(length) = length else {
                continue;
            };
            disk.push_back(Blocks {
                file_id: Some(file_id as u32),
                start_pos,
                length,
            });
            start_pos += length;
            let Some(length) = space else {
                continue;
            };
            disk.push_back(Blocks {
                file_id: None,
                start_pos,
                length,
            });
            start_pos += length;
        }
        Self(disk)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_sum() {
        let blocks = Blocks {
            start_pos: 10,
            length: 3,
            file_id: None,
        };
        assert_eq!(blocks.position_sum(), 33);
    }

    #[test]
    fn full_example() -> Result<()> {
        let example = "2333133121414131402";
        assert_eq!(compacted_checksum(example)?, 1928);
        Ok(())
    }
}
