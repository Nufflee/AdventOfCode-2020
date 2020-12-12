use num_derive::FromPrimitive;

use std::{
  iter::Sum,
  ops::{Add, AddAssign, Mul},
  str::FromStr,
};

#[derive(Debug)]
pub enum Instruction {
  Direction(Direction, i32),
  Foward(i32),
  Right(i32),
  Left(i32),
}

impl FromStr for Instruction {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::Direction::*;
    use Instruction::*;

    let (letter, value) = s.split_at(1);

    let value: i32 = value.parse().unwrap();

    match letter {
      "N" => Ok(Direction(North, value)),
      "S" => Ok(Direction(South, value)),
      "W" => Ok(Direction(West, value)),
      "E" => Ok(Direction(East, value)),
      "F" => Ok(Foward(value)),
      "R" => Ok(Right(value)),
      "L" => Ok(Left(value)),
      _ => Err("unknown instruction"),
    }
  }
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum Direction {
  West,
  North,
  East,
  South,
}

impl Direction {
  pub fn get_delta_position(&self) -> Pos {
    use Direction::*;

    match self {
      North => Pos(0, 1),
      South => Pos(0, -1),
      West => Pos(1, 0),
      East => Pos(-1, 0),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Pos(pub i32, pub i32);

impl Pos {
  pub fn manhattan_distance(self) -> i32 {
    self.0.abs() + self.1.abs()
  }
}

impl Add for Pos {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Pos(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl AddAssign for Pos {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
    self.1 += rhs.1;
  }
}

impl Mul<i32> for Pos {
  type Output = Self;

  fn mul(self, rhs: i32) -> Self::Output {
    Pos(self.0 * rhs, self.1 * rhs)
  }
}

impl Sum for Pos {
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(Pos(0, 0), Add::add)
  }
}
