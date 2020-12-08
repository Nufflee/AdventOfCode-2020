use std::{fmt::Display, str::FromStr};

// TODO: Try to make a stringification macro
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
  ACC { value: i32 },
  JMP { offset: i32 },
  NOP { value: i32 },
}

impl Display for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Instruction::ACC { value } => write!(f, "acc {}", value),
      Instruction::JMP { offset } => write!(f, "jmp {}", offset),
      Instruction::NOP { value } => write!(f, "nop {}", value),
    }
  }
}

impl FromStr for Instruction {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<_> = s.split_whitespace().collect();
    let arg: i32 = parts[1].parse().unwrap();

    match parts[0] {
      "acc" => Ok(Instruction::ACC { value: arg }),
      "jmp" => Ok(Instruction::JMP { offset: arg }),
      "nop" => Ok(Instruction::NOP { value: arg }),
      _ => Err("invalid instruction"),
    }
  }
}

pub struct ExecutionResult {
  pub infinite_loop_detected: bool,
  pub ip: usize,
  pub accumulator: i32,
}

pub fn execute(program: &[Instruction]) -> ExecutionResult {
  let mut accumulator = 0;
  let mut ip: usize = 0;
  let mut ran_instructions: Vec<usize> = Vec::new();
  let mut infinite_loop_detected = false;

  loop {
    if ran_instructions.contains(&ip) {
      infinite_loop_detected = true;
      break;
    }

    if ip >= program.len() {
      break;
    }

    let instruction = &program[ip];

    ran_instructions.push(ip);

    match instruction {
      Instruction::ACC { value } => {
        accumulator += value;
        ip += 1;
      }
      Instruction::JMP { offset } => {
        ip = ((ip as i32) + offset) as usize;
      }
      Instruction::NOP { .. } => {
        ip += 1;
      }
    }
  }

  ExecutionResult {
    infinite_loop_detected,
    ip,
    accumulator,
  }
}
