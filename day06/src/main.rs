use solution::Part;
use std::fs;

mod parser;
mod solution;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let groups: Vec<_> = contents.split("\n\n").map(parser::parse_group).collect();

  println!("Part 1:");
  println!(
    "\tSum of question counts is {}.",
    solution::solve(&groups, Part::One)
  );

  println!("Part 2:");
  println!(
    "\tSum of question counts is {}.",
    solution::solve(&groups, Part::Two)
  );

  Ok(())
}
