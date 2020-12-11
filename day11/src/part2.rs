use crate::sim;

pub fn solution(map: &[Vec<char>]) -> usize {
  sim::run(map, 5, true)
}
