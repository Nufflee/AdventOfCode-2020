use std::collections::HashMap;

pub type Passport = HashMap<String, String>;

pub fn parse_passport(passport_string: &str) -> Passport {
  let pairs = passport_string.split_whitespace();

  pairs
    .map(|pair| {
      let parts: Vec<_> = pair.split(':').collect();

      (parts[0].to_string(), parts[1].to_string())
    })
    .collect()
}
