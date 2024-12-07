use std::io;

use aoc2024_04_1::{num_xmas_hits, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = num_xmas_hits(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
