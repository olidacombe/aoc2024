use std::io;

use aoc2024_02_1::{num_safe_reports, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = num_safe_reports(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
