use crate::validators::validate_keys;
use crate::{parser::Passport, validators::VALIDATORS};

pub fn solution(passports: &[Passport]) -> usize {
  let passports_with_valid_keys = passports.iter().filter(|p| validate_keys(p));

  let valid_passports = passports_with_valid_keys.filter(|p| {
    VALIDATORS
      .iter()
      .all(|(key, validator)| validator(p.get(&key.to_string()).unwrap()))
  });

  valid_passports.count()
}
