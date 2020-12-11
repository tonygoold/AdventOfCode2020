use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut ns = Vec::new();
  for line in lines {
    let n = isize::from_str_radix(&line, 10).expect("Non-numeric input");
    if ns.len() < 25 {
      ns.push(n);
      continue;
    }
    let iter = ns.iter();
    let mut valid = false;
    'outer: for n1 in ns.iter() {
      for n2 in iter.clone() {
        if n1 != n2 && n == n1 + n2 {
          valid = true;
          break 'outer;
        }
      }
    }
    if !valid {
      println!("Found an invalid number: {}", n);
      break;
    }
    ns.remove(0);
    ns.push(n);
  }
}
