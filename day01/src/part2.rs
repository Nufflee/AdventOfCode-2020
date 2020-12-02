pub fn n_cubed_solution(numbers: &[i32]) -> i32 {
  for i in numbers {
    for j in numbers {
      for k in numbers {
        if i + j + k == 2020 {
          return i * j * k;
        }
      }
    }
  }

  -1
}

pub fn better_solution(numbers: &[i32]) -> i32 {
  for i in 0..numbers.len() {
    for j in (i + 1)..numbers.len() {
      for k in (j + 1)..numbers.len() {
        let a = numbers[i];
        let b = numbers[j];
        let c = numbers[k];

        if a + b + c == 2020 {
          return a * b * c;
        }
      }
    }
  }

  -1
}

pub fn rusty_better_solution(numbers: &[i32]) -> Option<i32> {
  for (i, a) in numbers.iter().enumerate() {
    for (j, b) in numbers.iter().skip(i).enumerate() {
      for c in numbers.iter().skip(j) {
        if a + b + c == 2020 {
          return Some(a * b * c);
        }
      }
    }
  }

  None
}
