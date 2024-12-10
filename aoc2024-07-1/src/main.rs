use std::io;

use aoc2024_07_1::{sum_achievable_test_values, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = sum_achievable_test_values(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
