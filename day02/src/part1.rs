use crate::parser::Policy;

pub fn solution(policies: &[Policy]) -> usize {
  policies
    .iter()
    .filter(|policy| {
      let occurences = policy.password.matches(policy.char).count();

      policy.range.contains(&occurences)
    })
    .count()
}
