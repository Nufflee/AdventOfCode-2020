pub fn n_squared_solution(numbers: &[i32]) -> i32 {
  for i in numbers {
    for j in numbers {
      if i + j == 2020 {
        return i * j;
      }
    }
  }

  -1
}

pub fn better_solution(numbers: &[i32]) -> i32 {
  for i in 0..numbers.len() {
    for j in (i + 1)..numbers.len() {
      let a = numbers[i];
      let b = numbers[j];

      if a + b == 2020 {
        return a * b;
      }
    }
  }

  -1
}

pub fn rusty_better_solution(numbers: &[i32]) -> Option<i32> {
  for (i, a) in numbers.iter().enumerate() {
    for b in numbers.iter().skip(i) {
      if a + b == 2020 {
        return Some(a * b);
      }
    }
  }

  None
}
