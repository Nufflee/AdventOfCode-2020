use crate::ship::{Instruction, Pos};

pub fn solution(instructions: &[Instruction]) -> i32 {
  let mut waypoint_position = Pos(-10, 1);

  instructions
    .iter()
    .map(|instruction| {
      use Instruction::*;

      let mut rotate_waypoint_position = |angle_in_deg| {
        let angle = std::f64::consts::FRAC_PI_2 * (angle_in_deg / 90) as f64;

        // Truncating to i32 is fine because angle is always a multiple of 90° or π/2
        let sin = angle.sin() as i32;
        let cos = angle.cos() as i32;

        waypoint_position = Pos(
          waypoint_position.0 * cos - waypoint_position.1 * sin,
          waypoint_position.0 * sin + waypoint_position.1 * cos,
        );
      };

      match instruction {
        Direction(direction, value) => {
          waypoint_position += direction.get_delta_position() * *value;
          Pos(0, 0)
        }
        Foward(value) => waypoint_position * *value,
        Right(value) => {
          rotate_waypoint_position(value);
          Pos(0, 0)
        }
        Left(value) => {
          rotate_waypoint_position(&-value);
          Pos(0, 0)
        }
      }
    })
    .sum::<Pos>()
    .manhattan_distance()
}
