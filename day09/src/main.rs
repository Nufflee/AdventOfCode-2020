use std::fs;

mod part1;
mod part2;

fn main() -> Result<(), std::io::Error> {
  let (input_path, is_example) = match std::env::args().nth(1).as_deref() {
    Some("-e") => ("input/example.txt", true),
    _ => ("input/test.txt", false),
  };

  let contents = fs::read_to_string(input_path)?;
  let numbers: Vec<u64> = contents.lines().map(|line| line.parse().unwrap()).collect();

  let preamble_length = if is_example { 5 } else { 25 };

  println!("Part 1:");
  println!(
    "\tFirst number that doesn't match: {}",
    part1::solution(&numbers, preamble_length)
  );

  println!("Part 2:");
  println!("\tSolution: {}", part2::solution(&numbers, preamble_length));

  Ok(())
}
