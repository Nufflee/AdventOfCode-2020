use crate::ship::{Direction, Instruction, Pos};
use num_traits::FromPrimitive;

pub fn solution(instructions: &[Instruction]) -> i32 {
  let mut orientation: Direction = Direction::East;

  instructions
    .iter()
    .map(|instruction| {
      use Instruction::*;

      match instruction {
        Direction(direction, value) => direction.get_delta_position() * *value,
        Foward(value) => orientation.get_delta_position() * *value,
        Right(value) => {
          orientation = FromPrimitive::from_i32((orientation as i32 + value / 90).rem_euclid(4)).unwrap();
          Pos(0, 0)
        }
        Left(value) => {
          orientation = FromPrimitive::from_i32((orientation as i32 - value / 90).rem_euclid(4)).unwrap();
          Pos(0, 0)
        }
      }
    })
    .sum::<Pos>()
    .manhattan_distance()
}
