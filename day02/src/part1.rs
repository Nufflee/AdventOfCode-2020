use super::parser::Policy;

pub fn solution(lines: &[&str]) -> usize {
  lines
    .iter()
    .filter(|line| {
      let policy = Policy::parse(line).unwrap();
      let occurences = policy.password.matches(policy.char).count();

      policy.range.contains(&occurences)
    })
    .count()
}
