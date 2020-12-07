use parser::{Bag, Rule};
use std::{
  collections::{HashMap, HashSet},
  fs,
};

mod parser;

type AdjacencyMap<'a> = HashMap<&'a String, Vec<String>>;

// TODO: REFACTOR THIS WHOLE FUCKIN THING

fn solution(rules: &[Rule]) -> u32 {
  let mut adjacency_map: AdjacencyMap = HashMap::new();

  for rule in rules {
    let node = match adjacency_map.get_mut(&rule.outer_bag) {
      Some(node) => node,
      None => {
        adjacency_map.insert(&rule.outer_bag, Vec::new());
        adjacency_map.get_mut(&rule.outer_bag).unwrap()
      }
    };

    node.append(
      &mut rule
        .inner_bags
        .iter()
        .map(|bag| bag.color.clone())
        .collect::<Vec<_>>(),
    )
  }

  let mut inverted_adjacency_map: AdjacencyMap = HashMap::new();

  for (key, value) in &adjacency_map {
    for v in value {
      if inverted_adjacency_map.contains_key(v) {
        inverted_adjacency_map.get_mut(v).unwrap().push(key.to_string());
      } else {
        inverted_adjacency_map.insert(v, vec![key.to_string()]);
      }
    }
  }

  //println!("{:#?}", inverted_adjacency_map);

  f(
    &inverted_adjacency_map,
    &mut HashSet::new(),
    inverted_adjacency_map.get(&"shiny gold".to_string()).unwrap(),
  )
}

fn new_map(rules: &[Rule]) -> u32 {
  let mut adjacency_map: HashMap<String, Vec<&Bag>> = HashMap::new();

  for rule in rules {
    let node = match adjacency_map.get_mut(&rule.outer_bag) {
      Some(node) => node,
      None => {
        adjacency_map.insert(rule.outer_bag.clone(), Vec::new());
        adjacency_map.get_mut(&rule.outer_bag).unwrap()
      }
    };

    node.append(&mut rule.inner_bags.iter().collect::<Vec<_>>())
  }

  /* for (key, value) in &adjacency_map {
    for v in value {
      println!("\"{}\" -> \"{}\" [label = {}]", key, v.color, v.quantity);
    }
  } */

  f2(&adjacency_map, &adjacency_map["shiny gold"], 1)
}

fn f(map: &AdjacencyMap, seen_nodes: &mut HashSet<String>, xs: &[String]) -> u32 {
  let mut result = 0;

  for x in xs {
    if seen_nodes.contains(&x.to_string()) {
      continue;
    }

    seen_nodes.insert(x.to_string());
    result += 1;

    if !map.contains_key(&x) {
      continue;
    }

    result += f(map, seen_nodes, map.get(&x).unwrap());
  }

  result
}

fn f2(map: &HashMap<String, Vec<&Bag>>, xs: &[&Bag], last_level: u32) -> u32 {
  let mut result = 0;

  for x in xs {
    /* if seen_nodes.contains(&x.to_string()) {
      continue;
    } */

    //seen_nodes.insert(x.to_string());
    result += x.quantity * last_level;

    if !map.contains_key(&x.color) {
      continue;
    }

    result += f2(map, map.get(&x.color).unwrap(), x.quantity * last_level);
  }

  result
}

fn main() -> Result<(), std::io::Error> {
  let input_path = match std::env::args().nth(1).as_deref() {
    Some("-e") => "input/example.txt",
    _ => "input/test.txt",
  };

  let contents = fs::read_to_string(input_path)?;
  let rules: Vec<_> = contents.lines().map(parser::parse_rule).collect();

  println!("Part 1:");
  println!("\tSolution: {}", solution(&rules));

  println!("Part 2:");
  println!("\tSolution: {}", new_map(&rules));

  Ok(())
}
