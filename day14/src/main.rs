use std::fs;

mod parser;
mod part1;
mod part2;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let instructions: Vec<parser::Instruction> = contents.lines().map(|line| line.parse().unwrap()).collect();

  println!("Part 1:");
  println!(
    "\tSum of all values in memory is {}.",
    part1::solution(&instructions)
  );

  println!("Part 2:");
  println!(
    "\tSum of all values in memory is {}.",
    part2::solution(&instructions)
  );

  Ok(())
}
