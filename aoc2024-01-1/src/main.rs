use std::io;

use aoc2024_01_1::{l2_distance_between_lists, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = l2_distance_between_lists(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
