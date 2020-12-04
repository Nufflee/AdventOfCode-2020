use std::collections::HashMap;

const REQUIRED_KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn solution(passports: &[HashMap<String, String>]) -> usize {
  passports.iter().filter(|p| validate_passport_keys(p)).count()
}

pub fn validate_passport_keys(passport: &HashMap<String, String>) -> bool {
  REQUIRED_KEYS.iter().all(|key| passport.contains_key(*key))
}
