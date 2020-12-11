use crate::sim;

pub fn solution(map: &[Vec<char>]) -> usize {
  sim::run(map, 4, false)
}
