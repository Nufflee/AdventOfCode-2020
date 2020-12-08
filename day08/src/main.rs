use std::fs;

mod part1;
mod part2;
mod vm;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let instructions: Vec<_> = contents.lines().map(|line| line.parse().unwrap()).collect();

  println!("Part 1:");
  println!(
    "\tValue in acc at time of infinite loop detection: {}",
    part1::solution(&instructions)
  );

  println!("Part 2:");
  println!(
    "\tValue in acc at time of termination: {}",
    part2::solution(&instructions)
  );

  Ok(())
}
