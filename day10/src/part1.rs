pub fn solution(joltage_ratings: &[u32]) -> u32 {
  let mut joltage_ratings = joltage_ratings.iter().copied().collect::<Vec<_>>();

  joltage_ratings.sort_unstable();

  // Outlet has effective jolt rating of 0
  joltage_ratings.insert(0, 0);

  let (one_jolt_diff, three_jolt_diff) = joltage_ratings.windows(2).fold((0u32, 1u32), |acc, x| {
    let diff = x[1] - x[0];

    (acc.0 + (diff == 1) as u32, acc.1 + (diff == 3) as u32)
  });

  one_jolt_diff * three_jolt_diff
}
