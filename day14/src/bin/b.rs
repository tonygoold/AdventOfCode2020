use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;

struct Mask {
  keep: u64,
  set: u64,
  split: Vec<u64>,
}

impl Mask {
  fn from_str(s: &str) -> Mask {
    let mut keep: u64 = 0;
    let mut set: u64 = 0;
    let mut split = Vec::new();
    let mut bit: u64 = 1 << (s.len() - 1);
    for c in s.chars() {
      match c {
        '0' => keep |= bit,
        '1' => set |= bit,
        'X' => split.push(bit),
        _ => panic!("Invalid character in mask"),
      }
      bit >>= 1;
    }
    Mask{keep, set, split}
  }

  fn apply (&self, x: u64) -> Vec<u64> {
    // Since we don't set the keep bit when we reach a split, we can assume the
    // initial bit value at each split is a zero, which simplifies this.
    let mut xs = vec![(x & self.keep) | self.set];
    for split in &self.split {
      let mut ys = xs.iter().map(|y| y | split).collect();
      xs.append(&mut ys);
    }
    xs
  }
}

struct Computer {
  memory: HashMap<u64, u64>,
  mask: Mask,
}

impl Computer {
  fn new(mask: Mask) -> Computer {
    Computer{
      memory: HashMap::new(),
      mask,
    }
  }

  fn set(&mut self, index: u64, value: u64) {
    for index in self.mask.apply(index) {
      self.memory.insert(index, value);
    }
  }

  fn sum(&self) -> u64 {
    self.memory.values().sum()
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let mut lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mask_re = Regex::new(r"^mask = (\w+)$")
    .expect("Unable to compile mask regex");
  let op_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$")
    .expect("Unable to compile op regex");

  // Assume the first line is always a mask, because no default is specified
  let mask = {
    let line = lines.next().expect("Missing mask line");
    let caps = mask_re.captures(&line)
      .expect("Failed to match mask line");
    Mask::from_str(&caps[1])
  };

  let mut computer = Computer::new(mask);
  for line in lines {
    if let Some(caps) = op_re.captures(&line) {
      let index = u64::from_str_radix(&caps[1], 10).expect("Invalid index");
      let value = u64::from_str_radix(&caps[2], 10).expect("Invalid value");
      computer.set(index, value);
    } else if let Some(caps) = mask_re.captures(&line) {
      computer.mask = Mask::from_str(&caps[1]);
    } else {
      panic!("Failed to match line");
    }
  }
  println!("Sum: {}", computer.sum());
}
