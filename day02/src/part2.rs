use super::parser::Policy;

pub fn solution(lines: &[&str]) -> usize {
  lines
    .iter()
    .filter(|line| {
      let parsed = Policy::parse(line).unwrap();
      let chars: Vec<_> = parsed.password.chars().collect();

      return (chars[parsed.range.start() - 1] == parsed.char) ^ (chars[parsed.range.end() - 1] == parsed.char);
    })
    .count()
}
