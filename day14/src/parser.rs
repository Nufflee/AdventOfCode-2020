use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
  SetMask { string_value: String },
  SetMemory { address: u64, value: u64 },
}

impl FromStr for Instruction {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with("mask") {
      let mask_string = s.split('=').nth(1).unwrap().trim();

      Ok(Instruction::SetMask {
        string_value: mask_string.to_string(),
      })
    } else {
      let parts = s.split('=').collect::<Vec<_>>();

      match parts.as_slice() {
        [beginning, value] => {
          let address = beginning.split('[').nth(1).unwrap().split(']').next().unwrap();

          Ok(Instruction::SetMemory {
            address: address.parse().unwrap(),
            value: value.trim().parse().unwrap(),
          })
        }
        _ => Err("invalid instruction"),
      }
    }
  }
}
