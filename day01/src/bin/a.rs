use std::io;
use std::io::{BufRead,BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
  let f = File::open("input.txt").expect("Unable to read input file");
  let reader = BufReader::new(f);
  let mut ns = Vec::new();

  let xs = reader.lines().into_iter().map(
    |x| usize::from_str_radix(&x.unwrap(), 10)
  );
  for x in xs {
    match x {
      Ok(n) => {
        let m = 2020 - n;
        if ns.contains(&m) {
          println!("Found a match: {:?} x {:?} = {:?}", m, n, m * n);
          return Ok(());
        }
        ns.push(n)
      },
      Err(err) => {
        println!("Unable to parse number: {:?}", err);
        return Ok(());
      }
    }
  }

  return Ok(());
}
