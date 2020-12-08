use crate::parser::Policy;

pub fn solution(policies: &[Policy]) -> usize {
  policies
    .iter()
    .filter(|policy| {
      let chars: Vec<_> = policy.password.chars().collect();

      return (chars[policy.range.start() - 1] == policy.char)
        ^ (chars[policy.range.end() - 1] == policy.char);
    })
    .count()
}
