use std::io::{BufRead,BufReader};
use std::fs::File;

const STEP_X: usize = 3;

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut x = 0;
  let mut hits = 0;
  for line in lines {
    let mut cs = line.chars().cycle();
    if cs.nth(x).unwrap() == '#' {
      hits += 1;
    }
    x += STEP_X;
  }
  println!("Hit {:?} trees", hits);
}
