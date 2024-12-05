use std::io::{self};

use aoc2024_03_2::{multiplication_sum, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let stdin = io::read_to_string(io::stdin())?;
    let answer = multiplication_sum(&stdin)?;
    println!("Answer: {answer}");
    Ok(())
}
