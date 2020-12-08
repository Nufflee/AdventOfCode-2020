use crate::parser::Passport;
use crate::validators;

pub fn solution(passports: &[Passport]) -> usize {
  passports.iter().filter(|p| validators::validate_keys(p)).count()
}
