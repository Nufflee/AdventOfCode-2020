use std::ops::RangeInclusive;

pub struct Policy {
  // Inspired by https://github.com/silen-z/advent-of-code/blob/main/src/2020/day02.rs#L26
  pub range: RangeInclusive<usize>,
  pub char: char,
  pub password: String,
}

impl Policy {
  pub fn parse(string: &str) -> Option<Self> {
    let dash_index = string.find('-')?;
    let space_index = string.find(' ')?;
    let colon_index = string.find(':')?;

    let start = string[0..dash_index].parse::<usize>().ok()?;
    let end = string[dash_index + 1..space_index].parse::<usize>().ok()?;
    let char = string[space_index + 1..colon_index].chars().next()?;
    let password = string[colon_index + 2..].to_string();

    Some(Self { range: start..=end, char, password })
  }
}
