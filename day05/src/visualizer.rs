use std::collections::HashSet;

// This is not a part of the original solution but something I came up with on my own
pub fn visualize(seat_ids: &[u32], my_seat: u32) -> String {
  let mut string = String::new();
  let seat_set: HashSet<_> = seat_ids.iter().collect();

  let min_id = seat_ids.iter().min().unwrap();
  let max_id = seat_ids.iter().max().unwrap();
  let width = 8 + 2 * 2 + 2;

  let wing_width = 45;
  let wing_height = 45;
  let wing_offset = 30;

  string += &create_nose(width, 2, wing_width as usize);

  for y in min_id / 8..max_id / 8 {
    let mut seat_string = (0..8).map(|x| {
      if seat_set.contains(&(y * 8 + x)) {
        "O"
      } else if y * 8 + x == my_seat {
        "\x1b[0;31mX\x1b[0m" // Red colored 'X'
      } else {
        "_"
      }
    });

    // Left wing
    if y > wing_offset && y < wing_offset + wing_width {
      string += &create_wing_line(
        (y - wing_offset) as usize,
        wing_width as usize,
        wing_height,
        Wing::Left,
      );
    } else {
      string += &" ".repeat(wing_width as usize);
    }

    let seat_iter = seat_string.by_ref();

    // Hull and seating
    string += &format!(
      "| {}  {}  {} |",
      seat_iter.take(3).collect::<String>(),
      seat_iter.take(2).collect::<String>(),
      seat_iter.take(3).collect::<String>()
    );

    // Right wing
    if y > wing_offset && y < wing_offset + wing_width {
      string += &create_wing_line(
        (y - wing_offset) as usize,
        wing_width as usize,
        wing_height,
        Wing::Right,
      )
    }

    string += "\n";
  }

  string += &create_tail(width, 4, wing_width as usize);

  string
}

#[derive(Eq, PartialEq)]
enum Wing {
  Right,
  Left,
}

fn create_wing_line(x: usize, width: usize, height: usize, wing: Wing) -> String {
  let mut string = String::new();

  if wing == Wing::Left {
    string += &format!("{:pad$}/", " ", pad = width - x);
  }

  if x == height - 1 {
    string += &"_".repeat(width - 2);
  } else {
    string += &" ".repeat(x - 1);
  }

  if wing == Wing::Right {
    string += "\\";
  }

  string
}

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

  string += &format!("{:pad$}{}\n", " ", "_".repeat(end_width), pad = pad + height);

  for i in (0..height).rev() {
    string += &format!("{:pad$}/{}\\\n", " ", " ".repeat(width - 2 * i), pad = pad + i);
  }

  string
}
