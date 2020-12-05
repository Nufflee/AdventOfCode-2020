use std::fs;

mod decoder;
mod part1;
mod part2;

/*
  TODO: Look into creating a custom wrapper type for SeatID (with From and To traits etc.)
    - How does this work in TS?
  TODO: Benchmark different solutions for part2
*/

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let boarding_passes: Vec<_> = contents.lines().map(decoder::decode_boarding_pass).collect();

  println!("Part 1:");
  println!("\tHighest Seat ID is: {}", part1::solution(&boarding_passes));
  println!("Part 2:");
  println!("\tMy Seat ID is per Sol. 1: {}", part2::solution1(&boarding_passes));
  println!("\tMy Seat ID is per Sol. 2: {}", part2::solution2(&boarding_passes));
  println!("\tMy Seat ID is per Sol. 3: {}", part2::solution3(&boarding_passes));

  Ok(())
}
