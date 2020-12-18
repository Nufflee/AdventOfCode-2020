use std::collections::HashMap;

use crate::parser::{Ticket, Validator};

pub fn solution(my_ticket: Ticket, validators: &[Validator], tickets: &[Ticket]) -> u64 {
  // First filter out all invalid tickets (ones where one or more fields don't match any validator)
  let valid_tickets = tickets
    .iter()
    .filter(|ticket| {
      !ticket
        .iter()
        .any(|field| !validators.iter().any(|validator| validator.is_valid(field)))
    })
    .collect::<Vec<_>>();

  let mut validator_to_fields: HashMap<&Validator, Vec<usize>> = HashMap::new();

  let column_count = tickets[0].len();

  // Then assemble a map of all fields (columns) that are valid for each validator
  for i in 0..column_count {
    for validator in validators {
      let mut is_column_valid = true;

      for ticket in &valid_tickets {
        if !validator.is_valid(&ticket[i]) {
          is_column_valid = false;
          break;
        }
      }

      if is_column_valid {
        validator_to_fields
          .entry(validator)
          .or_insert_with(Vec::new)
          .push(i);
      }
    }
  }

  let mut field_to_validator: HashMap<usize, &Validator> = HashMap::new();

  // Since we can only have 1 validator per field, we need to go through all of them again to establish this 1:1 mapping
  loop {
    for (validator, fields) in &validator_to_fields {
      let mut remaining_fields = Vec::new();

      for field in fields {
        if !field_to_validator.contains_key(field) {
          remaining_fields.push(field)
        }
      }

      if remaining_fields.len() == 1 {
        field_to_validator.insert(*remaining_fields[0], validator);
      }
    }

    if field_to_validator.len() == validators.len() {
      break;
    }
  }

  let mut result: u64 = 1;

  let departure_fields = field_to_validator
    .iter()
    .filter(|(_, validator)| validator.name.contains("departure"))
    .collect::<HashMap<_, _>>();

  for (i, _) in departure_fields {
    result *= my_ticket[*i] as u64;
  }

  result
}
