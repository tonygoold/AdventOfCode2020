use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;

struct Mask {
  keep: u64,
  set: u64,
}

impl Mask {
  fn from_str(s: &str) -> Mask {
    let mut keep: u64 = 0;
    let mut set: u64 = 0;
    for c in s.chars() {
      keep <<= 1;
      set <<= 1;
      match c {
        'X' => { keep |= 1; },
        '0' => {},
        '1' => { set |= 1; },
        _ => panic!("Invalid character in mask"),
      }
    }
    Mask{keep, set}
  }

  fn apply (&self, x: u64) -> u64 {
    (x & self.keep) | self.set
  }
}

struct Computer {
  memory: Vec<u64>,
  mask: Mask,
}

impl Computer {
  fn new(mask: Mask) -> Computer {
    Computer{
      memory: Vec::new(),
      mask,
    }
  }

  fn set(&mut self, index: usize, value: u64) {
    if index + 1 > self.memory.len() {
      self.memory.resize(index + 1, 0);
    }
    self.memory[index] = self.mask.apply(value);
  }

  fn sum(&self) -> u64 {
    self.memory.iter().sum()
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
      let index = usize::from_str_radix(&caps[1], 10).expect("Invalid index");
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
