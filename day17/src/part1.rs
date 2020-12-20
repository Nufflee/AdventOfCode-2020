use std::cmp;
use std::collections::HashSet;

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Vector3 {
  x: i32,
  y: i32,
  z: i32,
}

impl Vector3 {
  fn new(x: i32, y: i32, z: i32) -> Self {
    Self { x, y, z }
  }
}

type Dimension = HashSet<Vector3>;

fn get_dimension_bounds(dimension: &Dimension) -> (Vector3, Vector3) {
  let mut min = Vector3::new(i32::MAX, i32::MAX, i32::MAX);
  let mut max = Vector3::new(i32::MIN, i32::MIN, i32::MIN);

  for tile in dimension {
    min.x = cmp::min(tile.x, min.x);
    min.y = cmp::min(tile.y, min.y);
    min.z = cmp::min(tile.z, min.z);

    max.x = cmp::max(tile.x, max.x);
    max.y = cmp::max(tile.y, max.y);
    max.z = cmp::max(tile.z, max.z);
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
            Some(Vector3::new(x as i32, y as i32, 0))
          } else {
            None
          }
        })
        .collect::<Vec<_>>()
    })
    .collect()
}

// TODO: There's gotta be a way to generalize this to n-dimensions and reuse the same code for part 2 instead of duplicating it.
pub fn solution(lines: &[&str]) -> usize {
  let start_time = std::time::Instant::now();

  let dimension = parse_dimension(&lines);

  let ADJACENT_POSITIONS = {
    // TODO: Can this be without for loops
    let mut positions = Vec::new();

    for z in -1..=1 {
      for y in -1..=1 {
        for x in -1..=1 {
          if x == 0 && y == 0 && z == 0 {
            continue;
          }
          positions.push(Vector3::new(x, y, z));
        }
      }
    }

    positions
  };

  let mut dimension: Dimension = dimension.iter().copied().collect();

  for _ in 0..6 {
    let mut next_dimension: Dimension = dimension.iter().copied().collect();

    let (min, max) = get_dimension_bounds(&dimension);

    for z in min.z - 1..=max.z + 1 {
      for y in min.y - 1..=max.y + 1 {
        for x in min.x - 1..=max.x + 1 {
          let active_count = ADJACENT_POSITIONS.iter().fold(0, |acc, dp| {
            match dimension.get(&Vector3::new(x + dp.x, y + dp.y, z + dp.z)) {
              Some(_) => acc + 1,
              None => acc,
            }
          });

          let pos = Vector3::new(x, y, z);
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

    dimension = next_dimension;
  }

  println!("\tIt took {:?}.", start_time.elapsed());

  dimension.len()
}
