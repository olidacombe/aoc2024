use std::io;

use aoc2024_05_1::{sum_middle_valid_updates, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_middle_valid_updates(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
