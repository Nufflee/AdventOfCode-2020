#[derive(Debug)]
pub struct Bag {
  pub color: String,
  pub quantity: u32,
}

#[derive(Debug)]
pub struct Rule {
  pub outer_bag: String,
  pub inner_bags: Vec<Bag>,
}

fn parse_color<'a>(words: &mut impl Iterator<Item = &'a str>) -> String {
  words.by_ref().take(2).collect::<Vec<&str>>().join(" ")
}

pub fn parse_rule(line: &str) -> Rule {
  let mut words = line.split_whitespace();

  let color = parse_color(&mut words);

  match words.nth(2) {
    Some("no") => Rule {
      outer_bag: color,
      inner_bags: Vec::new(),
    },
    Some(mut quantity_str) => {
      let mut inner_bags = Vec::new();

      loop {
        inner_bags.push(Bag {
          quantity: quantity_str.parse().unwrap(),
          color: parse_color(&mut words),
        });

        if !words.next().unwrap().ends_with(',') {
          break;
        }

        quantity_str = words.by_ref().take(1).next().unwrap();
      }

      Rule {
        outer_bag: color,
        inner_bags,
      }
    }
    None => panic!("none?"),
  }
}
