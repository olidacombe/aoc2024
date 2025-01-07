use std::{collections::HashMap, hash::Hash, ops::AddAssign};

use common::parse::{self};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn fence_price(it: impl Iterator<Item = String>) -> Result<u64> {
    let mut partition = Partition::default();
    for (row, line) in it.enumerate() {
        for (col, crop) in line.chars().enumerate() {
            let vertex = Vertex {
                x: col as u64,
                y: row as u64,
            };
            partition.push_plot(vertex, crop);
        }
    }
    Ok(partition.total_price() as u64)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vertex {
    x: u64,
    y: u64,
}

impl Vertex {
    fn plot_edges(&self) -> [Edge; 4] {
        let Vertex { x, y } = self;
        let top_left = *self;
        let top_right = Vertex { x: x + 1, y: *y };
        let bottom_left = Vertex { x: *x, y: y + 1 };
        let bottom_right = Vertex { x: x + 1, y: y + 1 };
        [
            Edge(top_left, top_right),
            Edge(top_left, bottom_left),
            Edge(top_right, bottom_right),
            Edge(bottom_left, bottom_right),
        ]
    }
}

#[derive(Eq, Clone, Copy)]
struct Edge(Vertex, Vertex);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Edge(v1, v2) = self;
        let Vertex { x: x1, y: y1 } = v1;
        let Vertex { x: x2, y: y2 } = v2;
        if x1 < x2 || x1 == x2 && y1 < y2 {
            x1.hash(state);
            y1.hash(state);
            x2.hash(state);
            y2.hash(state);
        } else {
            x2.hash(state);
            y2.hash(state);
            x1.hash(state);
            y1.hash(state);
        }
    }
}

struct Region {
    area: usize,
    edges: HashMap<Edge, usize>, // edge -> count
}

impl Region {
    fn num_sides(&self) -> usize {
        todo!()
    }

    fn intersects(&self, edge: &Edge) -> bool {
        self.edges.contains_key(edge)
    }

    fn price(&self) -> usize {
        self.area * self.num_sides()
    }
}

impl AddAssign for Region {
    fn add_assign(&mut self, other: Self) {
        self.area += other.area;
        for (edge, count) in other.edges {
            *self.edges.entry(edge).or_default() += count;
        }
    }
}

#[derive(Default)]
struct Partition(HashMap<char, Vec<Region>>);

impl Partition {
    fn pop_incident_regions(&mut self, region: &Region, crop: char) -> Vec<Region> {
        let Some(candidate_regions) = self.0.remove(&crop) else {
            return Vec::new();
        };
        let (incident, disjoint): (Vec<_>, Vec<_>) = candidate_regions
            .into_iter()
            .partition(|candidate| region.edges.keys().any(|edge| candidate.intersects(edge)));
        self.0.insert(crop, disjoint);
        incident
    }

    fn push_plot(&mut self, top_left_vertex: Vertex, crop: char) {
        let mut region = Region {
            area: 1,
            edges: top_left_vertex
                .plot_edges()
                .iter()
                .cloned()
                .map(|edge| (edge, 1))
                .collect(),
        };
        for incident_region in self.pop_incident_regions(&region, crop) {
            region += incident_region;
        }
        self.0.entry(crop).or_default().push(region);
    }

    fn total_price(&self) -> usize {
        self.0
            .values()
            .map(|regions| regions.iter().map(Region::price).sum::<usize>())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn tiny_full_example() -> Result<()> {
        let example = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        assert_eq!(fence_price(example.lines().map(String::from))?, 80);
        Ok(())
    }

    #[test]
    fn small_example() -> Result<()> {
        let example = indoc! {"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        "};
        assert_eq!(fence_price(example.lines().map(String::from))?, 436);
        Ok(())
    }

    #[test]
    fn small_example_2() -> Result<()> {
        let example = indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "};
        assert_eq!(fence_price(example.lines().map(String::from))?, 368);
        Ok(())
    }

    #[test]
    fn example_e() -> Result<()> {
        let example = indoc! {"
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
        "};
        assert_eq!(fence_price(example.lines().map(String::from))?, 204);
        Ok(())
    }

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};
        assert_eq!(fence_price(example.lines().map(String::from))?, 1206);
        Ok(())
    }
}
