use super::part1;
use std::ops::Add;

pub fn solution(numbers: &[u64], preamble_length: usize) -> u64 {
  for (i, result) in numbers.iter().enumerate().skip(preamble_length) {
    let preamble = &numbers[(i - preamble_length)..i];

    if part1::find_two_numbers_that_sum_to(*result, preamble).is_none() {
      return *result;
    }
  }

  panic!("solution not found");
}

fn find_two_numbers_that_sum_to<T>(sum: T, numbers: &[T]) -> Option<(T, T)>
where
  T: PartialEq + Add<Output = T> + Copy,
{
  for (i, num_i) in numbers.iter().enumerate() {
    for num_j in numbers.iter().skip(i + 1) {
      if num_i != num_j && *num_i + *num_j == sum {
        return Some((*num_i, *num_j));
      }
    }
  }

  None
}
