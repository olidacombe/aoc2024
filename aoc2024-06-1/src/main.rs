use std::io;

use aoc2024_06_1::{num_distinct_guard_positions, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = num_distinct_guard_positions(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
