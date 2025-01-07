use std::io;

use aoc2024_12_2::{fence_price, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = fence_price(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
