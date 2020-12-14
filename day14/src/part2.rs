use crate::parser::Instruction;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Bitmask {
  mask: u64,
  values: Vec<u64>,
}

pub fn solution(instructions: &[Instruction]) -> u64 {
  let mut memory: HashMap<u64, u64> = HashMap::new();
  let mut current_mask = Bitmask::default();

  for instruction in instructions {
    match instruction {
      Instruction::SetMemory { address, value } => {
        for mask_value in &current_mask.values {
          let addr = (address & !current_mask.mask) | mask_value;
          memory.insert(addr, *value);
        }
      }
      Instruction::SetMask { string_value } => {
        current_mask = Bitmask::default();

        let mut floating_bits = 0;
        let mut floating_mask = 0;
        let mut value = 0;

        for (i, char) in string_value.chars().rev().enumerate() {
          if char == '1' {
            value |= 1 << i;
            current_mask.mask |= 1 << i;
          } else if char == 'X' {
            current_mask.mask |= 1 << i;

            floating_mask |= 1 << i;
            floating_bits += 1;
          }
        }

        // Calculate all bit combinations and superpose them on top of the original value
        for num in 0..2u64.pow(floating_bits) {
          let mut i = 0;
          let mut new_value = value;

          for j in 0..36 {
            if (floating_mask & (1u64 << j)) != 0 {
              new_value |= ((num >> i) & 1) << j;

              i += 1;
            }
          }

          current_mask.values.push(new_value);
        }
      }
    }
  }

  memory.values().sum()
}
