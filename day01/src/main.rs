use std::env;
use std::fs;

mod part1;
mod part2;

fn main() {
  let input_path = env::args().nth(1).unwrap_or_else(|| "input/input.txt".to_string());

  let contents = fs::read_to_string(input_path).unwrap();
  let numbers: Vec<i32> = contents.lines().map(|line| line.parse().unwrap()).collect();

  println!("Part 1:");
  println!("\tO(n²): {}", part1::n_squared_solution(&numbers));
  println!("\tO(n²/2): {}", part1::better_solution(&numbers));
  println!("\trusty O(n²/2): {:?}", part1::rusty_better_solution(&numbers));

  println!("Part 2:");
  println!("\tO(n³): {}", part2::n_cubed_solution(&numbers));
  println!("\tO(n³/6): {}", part2::better_solution(&numbers));
  println!("\trusty O(n³/6): {:?}", part2::rusty_better_solution(&numbers));
}
