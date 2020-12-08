use crate::vm;
use crate::vm::Instruction;

fn switch_instruction(instruction: Instruction) -> Instruction {
  match instruction {
    Instruction::JMP { offset } => Instruction::NOP { value: offset },
    Instruction::NOP { value } => Instruction::JMP { offset: value },
    _ => instruction,
  }
}

// TODO: Is there a non-bruteforce sol'n for this?
pub fn solution(instructions: &[Instruction]) -> i32 {
  let mut instructions = instructions.iter().copied().collect::<Vec<_>>();

  for i in 0..instructions.len() {
    let instruction = instructions[i];

    if matches!(instruction, Instruction::ACC { .. }) {
      continue;
    }

    instructions[i] = switch_instruction(instruction);

    let result = vm::execute(&instructions);

    instructions[i] = switch_instruction(instructions[i]);

    if !result.infinite_loop_detected {
      return result.accumulator;
    }
  }

  panic!("result not found");
}
