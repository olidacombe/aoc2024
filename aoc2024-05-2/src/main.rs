use std::io;

use aoc2024_05_2::{sum_middle_fixed_updates, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = sum_middle_fixed_updates(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
