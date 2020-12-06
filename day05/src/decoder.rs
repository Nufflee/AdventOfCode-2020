fn decode_binary_space(xs: &[bool], range: u32) -> u32 {
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
  let (row, column) = pass.split_at(7);

  let row = decode_binary_space(&row.chars().map(|c| c == 'F').collect::<Vec<_>>(), 128);
  let column = decode_binary_space(&column.chars().map(|c| c == 'L').collect::<Vec<_>>(), 8);

  row * 8 + column
}
