use std::fs;

mod parser;
mod part1;
mod part2;

use parser::*;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let lines = contents
    .lines()
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();

  let (my_ticket, nearby_tickets, validators) = parse(&lines);

  println!("Part 1:");
  println!(
    "\tTicket scanning error rate is {}.",
    part1::solution(&validators, &nearby_tickets)
  );

  println!("Part 2:");
  println!(
    "\tSolution is {}.",
    part2::solution(my_ticket, &validators, &nearby_tickets)
  );

  Ok(())
}
