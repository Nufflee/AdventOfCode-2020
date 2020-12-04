use std::collections::HashMap;

use super::part1;

pub fn solution(passports: &[HashMap<String, String>]) -> usize {
  let passports_with_valid_keys = passports.iter().filter(|p| part1::validate_passport_keys(p));

  let valid_passports = passports_with_valid_keys.filter(|p| validate_passport_values(p));

  valid_passports.count()
}

fn validate_passport_values(passport: &HashMap<String, String>) -> bool {
  match passport["byr"].parse::<u32>() {
    Ok(1920..=2002) => {}
    _ => return false,
  };

  match passport["iyr"].parse::<u32>() {
    Ok(2010..=2020) => {}
    _ => return false,
  };

  match passport["eyr"].parse::<u32>() {
    Ok(2020..=2030) => {}
    _ => return false,
  };

  let height = &passport["hgt"];

  match height[0..height.len() - 2].parse::<u32>() {
    Ok(150..=193) if height.ends_with("cm") => {}
    Ok(59..=76) if height.ends_with("in") => {}
    _ => return false,
  };

  let hair_color = &passport["hcl"];

  if hair_color.len() != 7 || !hair_color.starts_with('#') || !hair_color.chars().skip(1).all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c)) {
    return false;
  }

  const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

  if !VALID_EYE_COLORS.contains(&passport["ecl"].as_str()) {
    return false;
  }

  let pid = &passport["pid"];

  if pid.len() == 9 {
    match pid.parse::<u32>() {
      Ok(_) => {}
      Err(_) => return false,
    }
  } else {
    return false;
  }

  true
}
