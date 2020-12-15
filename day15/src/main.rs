use std::{collections::HashMap, fs};

fn solution(starting_numbers: &[u32], wanted_number: usize) -> u32 {
  let mut last_number: u32 = 0;
  let mut latest_index_map: HashMap<u32, usize> = HashMap::new();

  let start_time = std::time::Instant::now();

  for i in 0..wanted_number {
    let new_number = if i < starting_numbers.len() {
      starting_numbers[i]
    } else if let Some(last_index) = latest_index_map.get(&last_number) {
      (i - last_index - 1) as u32
    } else {
      0
    };

    if i > 0 {
      latest_index_map.insert(last_number, i - 1);
    }

    last_number = new_number;
  }

  println!("\tIt took: {:?}", start_time.elapsed());

  last_number
}

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let numbers: Vec<u32> = contents
    .lines()
    .next()
    .unwrap()
    .split(',')
    .map(|part| part.parse().unwrap())
    .collect();

  println!("Part 1:");
  println!("\t2020th number is {}.", solution(&numbers, 2020));

  println!("Part 2:");
  println!("\t30000000th number is {}.", solution(&numbers, 30000000));

  Ok(())
}
