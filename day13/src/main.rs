use std::fs;

mod part1;
mod part2;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let mut lines = contents.lines();

  let arrival_timestamp: u32 = lines.by_ref().next().unwrap().parse().unwrap();
  let buses: Vec<Option<u32>> = lines
    .next()
    .unwrap()
    .split(',')
    .map(|line| line.parse().ok())
    .collect();

  println!("Part 1:");
  println!(
    "\tBest bus ID * wait time = {}.",
    part1::solution(arrival_timestamp, &buses)
  );

  Ok(())
}
