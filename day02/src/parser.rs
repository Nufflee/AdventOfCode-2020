use std::str::FromStr;
use std::{num::ParseIntError, ops::RangeInclusive};

pub struct Policy {
  // Inspired by https://github.com/silen-z/advent-of-code/blob/main/src/2020/day02.rs#L26
  pub range: RangeInclusive<usize>,
  pub char: char,
  pub password: String,
}

impl FromStr for Policy {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<_> = s.split(' ').collect();

    match parts.as_slice() {
      [range, c, password] => {
        let range_parts: Vec<_> = range.split('-').collect();

        let range = range_parts[0].parse().unwrap()..=range_parts[1].parse().unwrap();

        return Ok(Self {
          range,
          char: c.chars().next().unwrap(),
          password: password.to_string(),
        });
      }
      _ => Err("u dumb lol"),
    }
  }
}
