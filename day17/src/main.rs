use std::fs;

mod part1;
mod part2;

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let lines = contents.lines().collect::<Vec<_>>();

  println!("Part 1:");
  println!(
    "\tNumber of active cubes after 6 cycles in 3D is {}.",
    part1::solution(&lines)
  );

  println!("Part 2:");
  println!(
    "\tNumber of active cubes after 6 cycles in 4D is {}.",
    part2::solution(&lines)
  );

  Ok(())
}
