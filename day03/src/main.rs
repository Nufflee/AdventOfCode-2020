use std::fs;

fn solution(map: &[Vec<char>], (dx, dy): (usize, usize)) -> u64 {
  let mut x = 0;
  let mut y = 0;
  let mut tree_count = 0;

  let width = map[0].len();
  let height = map.len();

  loop {
    x += dx;
    y += dy;

    x %= width;

    if y >= height {
      break;
    }

    if map[y][x] == '#' {
      tree_count += 1;
    }
  }

  tree_count
}

fn main() -> Result<(), std::io::Error> {
  let contents = fs::read_to_string("input/input.txt")?;
  let lines = contents.lines();

  let map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

  let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  let product: u64 = slopes.iter().map(|slope| solution(&map, *slope)).product();

  println!("Part 1:");
  println!("\tTrees encountered: {}", solution(&map, (3, 1)));
  println!("Part 2:");
  println!("\tTree product: {}", product);

  Ok(())
}
