use std::{iter::Sum, ops::Add};

pub fn solution(numbers: &[u64], preamble_length: usize) -> u64 {
  let invalid_number = super::part1::solution(numbers, preamble_length);
  let mut sequence = find_contigous_sequence_that_sums_to(invalid_number, numbers).unwrap();

  sequence.sort_unstable();

  sequence.first().unwrap() + sequence.last().unwrap()
}

fn find_contigous_sequence_that_sums_to<T>(target: T, numbers: &[T]) -> Option<Vec<T>>
where
  T: PartialEq + PartialOrd + Add<Output = T> + Sum<T> + Copy,
{
  for (i, num_i) in numbers.iter().enumerate() {
    let mut result = vec![*num_i];

    for num_j in numbers.iter().skip(i + 1) {
      result.push(*num_j);

      let sum = result.iter().copied().sum::<T>();

      if sum > target {
        break;
      }

      if sum == target {
        return Some(result);
      }
    }
  }

  None
}
