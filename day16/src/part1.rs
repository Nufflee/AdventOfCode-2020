use crate::parser::{Ticket, Validator};

pub fn solution(validators: &[Validator], tickets: &[Ticket]) -> u32 {
  tickets.iter().fold(0, |acc, ticket| {
    acc
      + ticket
        .iter()
        .filter(|field| !validators.iter().any(|validator| validator.is_valid(field)))
        .sum::<u32>()
  })
}
