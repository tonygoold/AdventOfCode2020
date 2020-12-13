use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

struct Tree {
  adapters: Vec<isize>,
  memos: HashMap<isize, usize>,
}

impl Tree {
  fn new(adapters: Vec<isize>) -> Tree {
    Tree{
      adapters,
      memos: HashMap::new(),
    }
  }

  fn num_sequences_base(&mut self) -> usize {
    let mut count = 0;
    for i in 0..self.adapters.len() {
      if self.adapters[i] <= 3 {
        count += self.num_sequences(i);
      } else {
        break;
      }
    }
    count
  }

  fn num_sequences(&mut self, index: usize) -> usize {
    if index + 1 == self.adapters.len() {
      return 1;
    }

    let adapter = self.adapters[index];
    let mut count = 0;
    match self.memos.get(&adapter) {
      Some(n) => return *n,
      None => {}
    }

    // Only the last adapter can complete the sequence so there are no special
    // termination cases.
    for i in index+1..self.adapters.len() {
      if self.adapters[i] <= adapter + 3 {
        count += self.num_sequences(i);
      } else {
        break
      }
    }
    self.memos.insert(adapter, count);
    count
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut ns: Vec<isize> = lines.map(
    |x| isize::from_str_radix(&x, 10).expect("Non-numeric input")
  ).collect();
  ns.sort();

  let mut tree = Tree::new(ns);
  let count = tree.num_sequences_base();
  println!("There are {} different sequences", count);
}
