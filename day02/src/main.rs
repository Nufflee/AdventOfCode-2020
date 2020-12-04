use std::fs;

use parser::Policy;

mod parser;
mod part1;
mod part2;

fn main() -> Result<(), std::io::Error> {
  let contents = fs::read_to_string("input/input.txt")?;

  let policies: Vec<Policy> = contents.lines().map(|line| line.parse().unwrap()).collect();

  println!("Part 1:");
  println!("\tValid passwords: {}", part1::solution(&policies));

  println!("Part 2:");
  println!("\tValid passwords: {}", part2::solution(&policies));

  Ok(())
}
