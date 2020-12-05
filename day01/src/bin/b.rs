use std::io;
use std::io::{BufRead,BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let ns = reader.lines().into_iter().map(
    |x| usize::from_str_radix(&x.unwrap(), 10).unwrap()
  ).collect::<Vec<usize>>();

  let mut candidates = Vec::new();
  for i in 0..ns.len() {
    for j in i..ns.len() {
      let ni = ns[i];
      let nj = ns[j];
      if ni + nj < 2020 {
        candidates.push((ni, nj));
      }
    }
  }

  for nk in ns {
    for candidate in &candidates {
      if nk + candidate.0 + candidate.1 == 2020 {
        let product = nk * candidate.0 * candidate.1;
        println!("{:?} x {:?} x {:?} = {:?}", candidate.0, candidate.1, nk, product);
        return Ok(());
      }
    }
  }

  println!("No matches found");
  return Ok(());
}
