use std::io;
use std::io::{BufRead,BufReader};
use std::fs::File;
use regex::Regex;

fn main() -> io::Result<()> {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut valid = 0;

  let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
  for line in lines {
    let matches = re.captures(&line).unwrap();
    let min = usize::from_str_radix(&matches[1], 10).unwrap();
    let max = usize::from_str_radix(&matches[2], 10).unwrap();
    let target = matches[3].chars().next().unwrap();
    let mut count = 0;
    for c in matches[4].chars() {
      if c == target {
        count += 1;
      }
    }
    if count >= min && count <= max {
      valid += 1;
    }
  }
  println!("{:?} valid", valid);

  Ok(())
}
