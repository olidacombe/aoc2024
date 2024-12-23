use std::io::{self};

use aoc2024_11_2::{num_stones, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let stdin = io::read_to_string(io::stdin())?;
    let answer = num_stones(&stdin, 75)?;
    println!("Answer: {answer}");
    Ok(())
}
