use std::collections::HashMap;

// TODO: Explore other solutions such as iterative DP and recursive but with subslices.
fn count_combinations_up_to(
  joltages: &[u32],
  memoization_map: &mut HashMap<usize, u64>,
  index: usize,
) -> u64 {
  if index == 0 {
    return 1;
  }

  if let Some(value) = memoization_map.get(&index) {
    return *value;
  }

  let mut result = 0;

  for j in (0..index).rev() {
    if joltages[index] - joltages[j] > 3 {
      break;
    }

    result += count_combinations_up_to(joltages, memoization_map, j);
  }

  memoization_map.insert(index, result);

  result
}

pub fn solution(joltage_ratings: &[u32]) -> u64 {
  let mut joltage_ratings = joltage_ratings.iter().copied().collect::<Vec<_>>();

  joltage_ratings.sort_unstable();

  // Outlet has effective jolt rating of 0
  joltage_ratings.insert(0, 0);

  // Laptop adapter has rating 3 higher than the highest adapter
  joltage_ratings.insert(joltage_ratings.len(), joltage_ratings.last().unwrap() + 3);

  count_combinations_up_to(&joltage_ratings, &mut HashMap::new(), joltage_ratings.len() - 1)
}
