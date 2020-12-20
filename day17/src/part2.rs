use std::cmp;
use std::collections::HashSet;

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Vector4 {
  x: i32,
  y: i32,
  z: i32,
  w: i32,
}

impl Vector4 {
  fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
    Self { x, y, z, w }
  }
}

type Dimension = HashSet<Vector4>;

fn get_dimension_bounds(dimension: &Dimension) -> (Vector4, Vector4) {
  let mut min = Vector4::new(i32::MAX, i32::MAX, i32::MAX, i32::MAX);
  let mut max = Vector4::new(i32::MIN, i32::MIN, i32::MIN, i32::MIN);

  for tile in dimension {
    min.x = cmp::min(tile.x, min.x);
    min.y = cmp::min(tile.y, min.y);
    min.z = cmp::min(tile.z, min.z);
    min.w = cmp::min(tile.w, min.w);

    max.x = cmp::max(tile.x, max.x);
    max.y = cmp::max(tile.y, max.y);
    max.z = cmp::max(tile.z, max.z);
    max.w = cmp::max(tile.w, max.w);
  }

  (min, max)
}

fn parse_dimension(lines: &[&str]) -> Dimension {
  lines
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
      line
        .chars()
        .enumerate()
        .filter_map(|(x, char)| {
          if char == '#' {
            Some(Vector4::new(x as i32, y as i32, 0, 0))
          } else {
            None
          }
        })
        .collect::<Vec<_>>()
    })
    .collect()
}

pub fn solution(lines: &[&str]) -> usize {
  let start_time = std::time::Instant::now();

  let dimension = parse_dimension(&lines);

  let ADJACENT_POSITIONS = {
    // TODO: Can this be without for loops
    let mut positions = Vec::new();

    for w in -1..=1 {
      for z in -1..=1 {
        for y in -1..=1 {
          for x in -1..=1 {
            if x == 0 && y == 0 && z == 0 && w == 0 {
              continue;
            }
            positions.push(Vector4::new(x, y, z, w));
          }
        }
      }
    }

    positions
  };

  println!("ADJACENT_POSITIONS.len() = {}", ADJACENT_POSITIONS.len());

  let mut dimension: Dimension = dimension.iter().copied().collect();

  for _ in 0..6 {
    let mut next_dimension: Dimension = dimension.iter().copied().collect();

    let (min, max) = get_dimension_bounds(&dimension);

    for w in min.w - 1..=max.w + 1 {
      for z in min.z - 1..=max.z + 1 {
        for y in min.y - 1..=max.y + 1 {
          for x in min.x - 1..=max.x + 1 {
            let active_count = ADJACENT_POSITIONS.iter().fold(0, |acc, dp| {
              match dimension.get(&Vector4::new(x + dp.x, y + dp.y, z + dp.z, w + dp.w)) {
                Some(_) => acc + 1,
                None => acc,
              }
            });

            let pos = Vector4::new(x, y, z, w);
            let is_active = dimension.get(&pos).is_some();

            if is_active {
              if active_count != 2 && active_count != 3 {
                next_dimension.remove(&pos);
              }
            } else if active_count == 3 {
              next_dimension.insert(pos);
            }
          }
        }
      }
    }

    dimension = next_dimension;
  }

  println!("\tIt took {:?}.", start_time.elapsed());

  dimension.len()
}
