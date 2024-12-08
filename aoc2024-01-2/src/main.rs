use std::io;

use aoc2024_01_2::{similarity_score, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = similarity_score(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
