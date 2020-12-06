use std::collections::HashSet;

pub enum Part {
  One,
  Two,
}

// TODO: Can I make this function nicer?
pub fn solve(groups: &[Vec<HashSet<char>>], part: Part) -> usize {
  groups
    .iter()
    .map(|group| {
      group
        .iter()
        .skip(1)
        .fold(group[0].clone(), |acc, person| match part {
          Part::One => acc.union(person).copied().collect(),
          Part::Two => acc.intersection(person).copied().collect(),
        })
    })
    .map(|set| set.len())
    .sum()
}
