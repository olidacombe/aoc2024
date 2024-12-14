use std::io;

use aoc2024_09_1::{compacted_checksum, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let stdin = io::read_to_string(io::stdin())?;
    let answer = compacted_checksum(&stdin)?;
    println!("Answer: {answer}");
    Ok(())
}
