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
  let passports: Vec<_> = contents.split("\n\n").map(parser::parse_passport).collect();

  println!("Part 1:");
  println!("\t{} passports are valid.", part1::solution(&passports));
  println!("Part 2:");
  println!("\t{} passports are valid.", part2::solution(&passports));

  Ok(())
}
