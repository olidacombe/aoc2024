use std::io;

use aoc2024_08_1::{num_antinodes, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = num_antinodes(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
