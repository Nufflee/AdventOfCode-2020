// This is not a part of the original solution but something I came up with on my own
use std::collections::HashSet;

pub fn visualize(seat_ids: &[u32], my_seat: u32) -> String {
  let mut string = String::new();
  let seat_set: HashSet<_> = seat_ids.iter().collect();

  let min_id = seat_ids.iter().min().unwrap();
  let max_id = seat_ids.iter().max().unwrap();
  let width = 8 + 2 * 2 + 2;

  let wing_width = 40;
  let wing_height = 40;
  let wing_offset = 30;

  string += &create_nose(width, 2, wing_width as usize);

  for x in min_id / 8..max_id / 8 {
    let mut seat_string = (0..8).map(|y| {
      if seat_set.contains(&(x * 8 + y)) {
        "O"
      } else if x * 8 + y == my_seat {
        "\x1b[0;31mX\x1b[0m" // Red colored 'X'
      } else {
        "_"
      }
    });

    // Left wing
    // TODO: Needs to be refactored into create_wing()
    if x > wing_offset && x < wing_offset + wing_width {
      let i = x - wing_offset;

      string += &format!("{:pad$}/", " ", pad = (wing_width - i) as usize);

      if i == wing_height - 1 {
        string += &"_".repeat((wing_width - 2) as usize);
      } else {
        string += &" ".repeat((i - 1) as usize)
      }
    } else {
      string += &" ".repeat(wing_width as usize);
    }

    // Hull and seating
    string += &format!(
      "| {}  {}  {} |",
      seat_string.by_ref().take(3).collect::<String>(),
      seat_string.by_ref().take(2).collect::<String>(),
      seat_string.by_ref().take(3).collect::<String>()
    );

    // Right wing
    // TODO: Needs to be refactored into create_wing()
    if x > wing_offset && x < wing_offset + wing_width {
      let i = x - wing_offset;

      if i == wing_height - 1 {
        string += &"_".repeat((wing_width - 2) as usize);
      } else {
        string += &" ".repeat((i - 1) as usize);
      }

      string += "\\";
    }

    string += "\n";
  }

  string += &create_tail(width, 4, wing_width as usize);

  string
}

// TODO: Consolidate with create_nose
fn create_tail(width: usize, end_width: usize, pad: usize) -> String {
  let mut string = String::new();

  assert!(end_width % 2 == 0);

  let height = width / 2 - end_width / 2 + 1;

  for i in 0..height {
    let end_char = if i == height - 1 { "_" } else { " " };

    string += &format!("{}\\{}/\n", " ".repeat(pad + i), end_char.repeat(width - 2 * i));
  }

  string
}

fn create_nose(width: usize, end_width: usize, pad: usize) -> String {
  let mut string = String::new();

  assert!(end_width % 2 == 0);

  let height = width / 2 - end_width / 2 + 1;

  string += &format!(
    "{}{}\n",
    " ".repeat(pad + width / 2 - end_width / 2 + 1),
    "_".repeat(end_width)
  );

  for i in (0..height).rev() {
    string += &format!("{}/{}\\\n", " ".repeat(pad + i), " ".repeat(width - 2 * i));
  }

  string
}
