#[macro_use]
extern crate lazy_static;

use std::fs;

mod part1;
mod part2;
mod sim;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let map: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();

  println!("Part 1:");
  println!(
    "\tOccupied seats at the end of simulation: {}",
    part1::solution(&map)
  );

  println!("Part 2:");
  println!(
    "\tOccupied seats at the end of simulation: {}",
    part2::solution(&map)
  );

  Ok(())
}
