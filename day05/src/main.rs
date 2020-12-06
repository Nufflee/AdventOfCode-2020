use core::panic;
use std::fs;

mod decoder;
mod part1;
mod part2;
mod visualizer;

/*
  TODO: Look into creating a custom wrapper type for SeatID (with From and To traits etc.)
    - How does this work in TS?
  TODO: Benchmark different solutions for part2
  TODO: Better argument parsing
    - It's currently impossible to have both "-e" and "-v" switches
*/

fn main() -> Result<(), std::io::Error> {
  let args: Vec<_> = std::env::args().collect();
  let mut input_path = "input/test.txt";
  let mut should_visualize = false;

  match args.len() {
    1 => {}
    2 => match args[1].as_str() {
      "-e" => input_path = "input/example.txt",
      "-v" => should_visualize = true,
      _ => input_path = args[1].as_str(),
    },
    _ => panic!("Usage: day05 <data_path> {-e|-v}"),
  }

  let contents = fs::read_to_string(input_path)?;
  let seat_ids: Vec<_> = contents.lines().map(decoder::decode_boarding_pass).collect();

  println!("Part 1:");
  println!("\tHighest Seat ID is: {}", part1::solution(&seat_ids));

  println!("Part 2:");
  let my_seat = part2::solution1(&seat_ids);

  println!("\tMy Seat ID is per Sol. 1: {}", my_seat);
  println!("\tMy Seat ID is per Sol. 2: {}", part2::solution2(&seat_ids));
  println!("\tMy Seat ID is per Sol. 3: {}", part2::solution3(&seat_ids));

  // This is not a part of the original solution but something I came up with on my own
  if should_visualize {
    println!("{}", visualizer::visualize(&seat_ids, my_seat));
  }

  Ok(())
}
