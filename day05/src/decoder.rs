fn reverse_binary_search(xs: &[bool], range: u32) -> u32 {
  let mut low = 0;
  let mut high = range - 1;

  for x in xs {
    if *x {
      high -= (high - low + 1) / 2;
    } else {
      low += (high - low + 1) / 2;
    }
  }

  assert!(low == high);

  low
}

pub fn decode_boarding_pass(pass: &str) -> u32 {
  let row = reverse_binary_search(&pass.chars().take(7).map(|c| c == 'F').collect::<Vec<_>>(), 128);
  let column = reverse_binary_search(&pass.chars().skip(7).map(|c| c == 'L').collect::<Vec<_>>(), 8);

  row * 8 + column
}
