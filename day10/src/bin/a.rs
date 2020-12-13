use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut ns: Vec<isize> = lines.map(
    |x| isize::from_str_radix(&x, 10).expect("Non-numeric input")
  ).collect();
  ns.sort();

  let mut diffs: HashMap<isize, isize> = HashMap::new();
  // The built-in adapter
  diffs.insert(3, 1);
  let mut prev = 0;
  for n in ns.iter() {
    let diff = *n - prev;
    if diff > 3 {
      println!("Unable to step from {} to {}", prev, *n);
      return;
    }
    prev = *n;
    *diffs.entry(diff).or_default() += 1;
  }

  let d1s = diffs.get(&1).copied().unwrap_or(0);
  let d3s = diffs.get(&3).copied().unwrap_or(0);
  println!("{} ones by {} threes = {}", d1s, d3s, d1s * d3s);
}
