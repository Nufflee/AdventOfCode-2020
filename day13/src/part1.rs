pub fn solution(arrival_timestamp: u32, buses: &[Option<u32>]) -> u32 {
  let mut buses = buses.iter().copied().collect::<Vec<_>>();

  buses.sort_unstable();

  for bus_id in buses {
    if let Some(bus_id) = bus_id {
      let a: u32 = (arrival_timestamp as f64 / bus_id as f64).round() as u32;
      let bus_arrival = a * bus_id;

      if bus_arrival >= arrival_timestamp {
        return bus_id * (bus_arrival - arrival_timestamp);
      }
    }
  }

  panic!("no solution found");
}
