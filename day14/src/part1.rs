use crate::parser::Instruction;
use std::collections::HashMap;
#[derive(Debug, Copy, Clone, Default)]
pub struct Bitmask {
  value: u64,
  mask: u64,
}

pub fn solution(instructions: &[Instruction]) -> u64 {
  let mut memory: HashMap<u64, u64> = HashMap::new();
  let mut current_mask = Bitmask::default();

  for instruction in instructions {
    match instruction {
      Instruction::SetMemory { address, value } => {
        memory.insert(*address, (value & !current_mask.mask) | current_mask.value);
      }
      Instruction::SetMask { string_value } => {
        current_mask = Bitmask::default();

        for (i, char) in string_value.chars().rev().enumerate() {
          if char != 'X' {
            current_mask.mask |= 1 << i;
            current_mask.value |= (char.to_digit(10).unwrap() as u64) << i;
          }
        }
      }
    }
  }

  memory.values().sum()
}
