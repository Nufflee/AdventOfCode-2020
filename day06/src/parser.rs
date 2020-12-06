use std::collections::HashSet;

pub fn parse_group(group: &str) -> Vec<HashSet<char>> {
  let people = group.split('\n');

  people.map(|person| person.chars().collect()).collect()
}
