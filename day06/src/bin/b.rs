use std::collections::HashMap;
use std::io::{BufRead,BufReader};
use std::fs::File;

struct Group {
  answers: usize,
  yeses: HashMap<char, usize>
}

impl Group {
  fn add(&mut self, c: char) {
    self.yeses.insert(c, self.yeses.get(&c).cloned().unwrap_or(0) + 1);
  }

  fn add_answer(&mut self, line: &str) {
    self.answers += 1;
    for c in line.chars() {
      self.add(c);
    }
  }

  fn count_all_yeses(&self) -> usize {
    self.yeses.values().filter(|x| *x == &self.answers).count()
  }

  fn new() -> Group {
    Group {
      answers: 0,
      yeses: HashMap::new()
    }
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut groups = Vec::new();
  let mut group = Group::new();
  for line in lines {
    if line == "" {
      groups.push(group);
      group = Group::new();
    } else {
      group.add_answer(&line);
    }
  }
  groups.push(group);

  let yeses = groups.iter().map(|x| x.count_all_yeses()).fold(0, |x,y| x+y);
  println!("Total number of questions answered yes by group: {}", yeses);
}
