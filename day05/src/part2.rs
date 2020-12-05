use std::collections::HashSet;

pub fn solution1(seat_ids: &[u32]) -> u32 {
  let set: HashSet<_> = seat_ids.iter().collect();

  let min_id = seat_ids.iter().min().unwrap();
  let max_id = seat_ids.iter().max().unwrap();

  for seat_id in seat_ids {
    if seat_id == min_id || seat_id == max_id {
      continue;
    }

    if !set.contains(&(seat_id + 1)) {
      return seat_id + 1;
    }
  }

  0
}

pub fn solution2(seat_ids: &[u32]) -> u32 {
  let mut seat_ids = seat_ids.to_vec();

  seat_ids.sort_unstable();

  for (i, seat_id) in seat_ids.iter().enumerate().skip(1) {
    if seat_id - seat_ids[i - 1] != 1 {
      return seat_id - 1;
    }
  }

  0
}

pub fn solution3(seat_ids: &[u32]) -> u32 {
  let min_id = seat_ids.iter().min().unwrap();
  let max_id = seat_ids.iter().max().unwrap();

  (*min_id..=*max_id).sum::<u32>() - seat_ids.iter().sum::<u32>()
}
