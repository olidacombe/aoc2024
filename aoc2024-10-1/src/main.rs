use std::io;

use aoc2024_10_1::{sum_trailhead_scores, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().map_while(|l| l.ok());
    let answer = sum_trailhead_scores(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
