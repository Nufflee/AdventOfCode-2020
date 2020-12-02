use std::fs;

mod parser;
mod part1;
mod part2;

fn main() -> Result<(), std::io::Error> {
  let contents = fs::read_to_string("input/input.txt")?;
  let lines: Vec<&str> = contents.lines().collect();

  println!("Part 1:");
  println!("\tValid passwords: {}", part1::solution(&lines));

  println!("Part 2:");
  println!("\tValid passwords: {}", part2::solution(&lines));

  Ok(())
}
