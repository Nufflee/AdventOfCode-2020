extern crate num_derive;

use ship::Instruction;
use std::fs;

mod part1;
mod part2;
mod ship;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let map: Vec<Instruction> = contents.lines().map(|line| line.parse().unwrap()).collect();

  println!("Part 1:");
  println!(
    "\tShip is {:?} units away from the origin.",
    part1::solution(&map)
  );

  println!("Part 2:");
  println!(
    "\tShip is {:?} units away from the origin.",
    part2::solution(&map)
  );

  Ok(())
}
