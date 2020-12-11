use std::fs::File;
use std::io::{BufRead,BufReader};

const TARGET: isize = 217430975;

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut ns = Vec::new();
  for line in lines {
    let n = isize::from_str_radix(&line, 10).expect("Non-numeric input");
    ns.push(n);
    let mut sum = ns.iter().fold(0, |x, y| x + y);
    while sum > TARGET {
      sum -= ns.remove(0);
    }
    if sum == TARGET {
      break;
    }
  }
  match (ns.iter().min(), ns.iter().max()) {
    (Some(min), Some(max)) => println!("{} + {} = {}", min, max, min + max),
    _ => println!("Failed to extract min and max")
  }
}
