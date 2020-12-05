use std::io::{BufRead,BufReader};
use std::fs::File;

fn count_hits(lines: &Vec<String>, step_x: usize, step_y: usize) -> usize {
  let mut x = 0;
  let mut y = 0;
  let mut hits = 0;
  while y < lines.len() {
    let mut cs = lines[y].chars().cycle();
    if cs.nth(x).unwrap() == '#' {
      hits += 1;
    }
    x += step_x;
    y += step_y;
  }
  hits
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap()).collect();

  let steps = [ (1, 1), (3, 1), (5, 1), (7, 1), (1, 2) ];
  let hits = steps.iter().map(|s| count_hits(&lines, s.0, s.1)).fold(1, |x, y| x*y);

  println!("Hit {:?} trees", hits);
}
