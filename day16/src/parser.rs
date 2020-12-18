use std::{ops::RangeInclusive, str::FromStr};

pub type Ticket = Vec<u32>;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Validator {
  pub name: String,
  pub range1: RangeInclusive<u32>,
  pub range2: RangeInclusive<u32>,
}

impl Validator {
  pub fn is_valid(&self, value: &u32) -> bool {
    self.range1.contains(value) || self.range2.contains(value)
  }
}

impl FromStr for Validator {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split(':').collect::<Vec<_>>().as_slice() {
      [name, ranges] => {
        let ranges = ranges
          .split("or")
          .map(|range| {
            match range
              .split('-')
              .map(|num| num.trim().parse().unwrap())
              .collect::<Vec<_>>()
              .as_slice()
            {
              [start, end] => *start..=*end,
              _ => panic!("invalid range"),
            }
          })
          .collect::<Vec<_>>();

        Ok(Self {
          name: name.to_string(),
          range1: ranges[0].clone(),
          range2: ranges[1].clone(),
        })
      }
      _ => Err("invalid validator"),
    }
  }
}

fn parse_ticket(line: &str) -> Ticket {
  line.split(',').map(|x| x.parse().unwrap()).collect()
}

// TODO: Can this be made more readable
pub fn parse(lines: &[&str]) -> (Ticket, Vec<Ticket>, Vec<Validator>) {
  let parts = lines.split(|line| line == &"your ticket:").collect::<Vec<_>>();
  let (validators, parts) = (
    parts[0]
      .iter()
      .map(|line| line.parse::<Validator>().unwrap())
      .collect::<Vec<_>>(),
    parts[1]
      .split(|line| line == &"nearby tickets:")
      .collect::<Vec<_>>(),
  );
  let (my_ticket, nearby_tickets) = (
    parse_ticket(parts[0][0]),
    parts[1].iter().map(|line| parse_ticket(line)).collect::<Vec<_>>(),
  );

  (my_ticket, nearby_tickets, validators)
}
