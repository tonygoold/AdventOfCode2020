use std::fmt;
use std::io::{BufRead,BufReader};
use std::fs::File;
use regex::Regex;

enum Op {
  Acc,
  Jmp,
  Nop,
}

struct Instruction {
  op: Op,
  value: isize,
  exec_count: usize,
}

impl Instruction {
  fn new(op: Op, value: isize) -> Instruction {
    Instruction{
      op,
      value,
      exec_count: 0,
    }
  }
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let op = match self.op {
      Op::Acc => "acc",
      Op::Jmp => "jmp",
      Op::Nop => "nop"
    };
    write!(f, "{} {}", op, self.value)
  }
}

struct Computer {
  acc: isize,
  instructions: Vec<Instruction>,
  iptr: isize,
}

impl Computer {
  fn new(instructions: Vec<Instruction>) -> Computer {
    Computer{
      acc: 0,
      instructions,
      iptr: 0,
    }
  }

  fn run(&mut self) -> isize {
    loop {
      let inst = &mut self.instructions[self.iptr as usize];
      if inst.exec_count > 0 {
        break
      }
      inst.exec_count += 1;
      match inst.op {
        Op::Acc => self.acc += inst.value,
        Op::Jmp => self.iptr += inst.value - 1,
        Op::Nop => {}
      }
      self.iptr += 1;
    }
    self.acc
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let line_re = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").expect("Unable to compile line regex");
  let mut instructions = Vec::new();
  for line in lines {
    let caps = line_re.captures(&line).expect("Line does not match pattern");
    let value = isize::from_str_radix(&caps[2], 10).expect("Invalid value");
    let op = match &caps[1] {
      "acc" => Op::Acc,
      "jmp" => Op::Jmp,
      "nop" => Op::Nop,
      _ => panic!("Invalid operation")
    };
    instructions.push(Instruction::new(op, value));
  }
  let mut computer = Computer::new(instructions);
  let acc = computer.run();
  println!("Accumulator after running: {}", acc);
}
