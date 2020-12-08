use super::vm;

pub fn solution(instructions: &[vm::Instruction]) -> i32 {
  vm::execute(instructions).accumulator
}
