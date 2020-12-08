use crate::parser::Passport;

type Validator<'a> = (&'a str, fn(&str) -> bool);

pub fn validate_keys(passport: &Passport) -> bool {
  VALIDATORS.iter().all(|(key, _)| passport.contains_key(*key))
}

pub const VALIDATORS: &[Validator] = &[
  ("byr", |byr| matches!(byr.parse(), Ok((1920..=2002)))),
  ("iyr", |iyr| matches!(iyr.parse(), Ok((2010..=2020)))),
  ("eyr", |eyr| matches!(eyr.parse(), Ok((2020..=2030)))),
  ("pid", |pid| matches!(pid.parse::<u32>(), Ok(_) if pid.len() == 9)),
  ("hgt", validate_hgt),
  ("hcl", validate_hcl),
  ("ecl", validate_ecl),
];

fn validate_hgt(height: &str) -> bool {
  let (height, unit) = height.split_at(height.len() - 2);

  match height.parse::<u32>() {
    Ok(150..=193) if unit == "cm" => true,
    Ok(59..=76) if unit == "in" => true,
    _ => false,
  }
}

fn validate_hcl(hcl: &str) -> bool {
  hcl.len() == 7
    && hcl.starts_with('#')
    && hcl
      .chars()
      .skip(1)
      .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
}

fn validate_ecl(ecl: &str) -> bool {
  const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

  VALID_EYE_COLORS.contains(&ecl)
}
