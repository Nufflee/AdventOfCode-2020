lazy_static! {
  static ref ADJACENT_POSITIONS: Vec<(i32, i32)> = {
    // TODO: Can this be without for loops
    let mut positions = Vec::new();

    for y in -1..=1 {
      for x in -1..=1 {
        if x == 0 && y == 0 {
          continue;
        }
        positions.push((y, x))
      }
    }

    positions
  };
}

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';

pub fn run(map: &[Vec<char>], occupation_threshold: usize, raytracing: bool) -> usize {
  let mut map = map.to_vec();

  let height = map.len();
  let width = map[0].len();

  loop {
    let mut next_map: Vec<Vec<char>> = (0..height).map(|_| vec!['\0'; width]).collect();

    for y in 0..height {
      for x in 0..width {
        // TODO: Try to replace this with a custom iterator.
        let occupied_seat_count = ADJACENT_POSITIONS
          .iter()
          .filter(|(dx, dy)| {
            // Distance the ray has passed from the current tile
            let mut distance = 1;

            // TODO: Refactor this loop
            loop {
              let check_x = (x as i32) + dx * distance;
              let check_y = (y as i32) + dy * distance;

              if check_x < 0 || check_y < 0 || check_x >= width as i32 || check_y >= height as i32 {
                return false;
              }

              let tile = map[check_y as usize][check_x as usize];

              if tile == OCCUPIED_SEAT {
                return true;
              } else if tile == EMPTY_SEAT {
                return false;
              }

              if !raytracing {
                return false;
              }

              distance += 1;
            }
          })
          .count();

        let tile = map[y][x];

        next_map[y][x] = match tile {
          EMPTY_SEAT if occupied_seat_count == 0 => OCCUPIED_SEAT,
          OCCUPIED_SEAT if occupied_seat_count >= occupation_threshold => EMPTY_SEAT,
          _ => tile,
        }
      }
    }

    if map == next_map {
      break;
    }

    map = next_map;
  }

  map
    .iter()
    .map(|row| row.iter().filter(|c| **c == OCCUPIED_SEAT).count())
    .sum()
}
