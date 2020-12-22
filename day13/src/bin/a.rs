use std::fs::File;
use std::io::{BufRead,BufReader};

struct Routing {
  earliest: isize,
  routes: Vec<Option<isize>>,
}

impl Routing {
  fn next_departure(&self, id: isize) -> isize {
    match self.earliest % id {
      0 => 0,
      r => id - r,
    }
  }

  fn earliest_bus(&self) -> Option<isize> {
    self.routes.iter().flatten()
      .min_by_key(|x| self.next_departure(**x))
      .cloned()
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let mut lines = reader.lines().into_iter().map(|x| x.unwrap());

  let earliest = isize::from_str_radix(
    &lines.next().expect("Missing earliest departure line"), 10
  ).expect("Invalid earliest departure line");
  let routes = lines.next().expect("Missing routes line").split(',').map(
    |x| match x {
      "x" => None,
      _ => Some(isize::from_str_radix(&x, 10).expect("Invalid bus id"))
    }
  ).collect();

  let routing = Routing{earliest, routes};
  match routing.earliest_bus() {
    Some(id) => {
      let departure = routing.next_departure(id);
      println!("{} departing in {} minutes -> {}", id, departure, id * departure)
    },
    None => println!("Apparently don't have any routes")
  }
}
