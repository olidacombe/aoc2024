use std::io;

use aoc2024_06_2::{num_loopy_obstruction_positions, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = num_loopy_obstruction_positions(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
