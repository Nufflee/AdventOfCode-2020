use std::fs;

mod part1;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let numbers: Vec<u32> = contents.lines().map(|line| line.parse().unwrap()).collect();

  println!("Product of differences: {}", part1::solution(&numbers));

  Ok(())
}
