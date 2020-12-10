use std::fmt;
use std::io::{BufRead,BufReader};
use std::fs::File;
use regex::Regex;

#[derive(Copy, Clone)]
enum Op {
  Acc,
  Jmp,
  Nop,
}

#[derive(Copy, Clone)]
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

struct Computer<'a> {
  acc: isize,
  instructions: &'a mut Vec<Instruction>,
  iptr: isize,
}

impl<'a> Computer<'a> {
  fn new(instructions: &'a mut Vec<Instruction>) -> Computer {
    Computer{
      acc: 0,
      instructions,
      iptr: 0,
    }
  }

  fn run(&mut self) -> Option<isize> {
    while (self.iptr as usize) != self.instructions.len() {
      let inst = &mut self.instructions[self.iptr as usize];
      if inst.exec_count > 0 {
        return None;
      }
      inst.exec_count += 1;
      match inst.op {
        Op::Acc => self.acc += inst.value,
        Op::Jmp => self.iptr += inst.value - 1,
        Op::Nop => {}
      }
      self.iptr += 1;
    }
    Some(self.acc)
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

  for i in 0..instructions.len() {
    let inst = &instructions[i];
    let acc = match inst.op {
      Op::Acc => None,
      Op::Jmp => {
        let mut altered = instructions.clone();
        altered[i] = Instruction::new(Op::Nop, inst.value);
        let mut computer = Computer::new(&mut altered);
        computer.run()
      },
      Op::Nop => {
        let mut altered = instructions.clone();
        altered[i] = Instruction::new(Op::Jmp, inst.value);
        let mut computer = Computer::new(&mut altered);
        computer.run()
      }
    };
    match acc {
      Some(val) => {
        println!("Accumulator after altering instruction {}: {}", i, val);
        return;
      },
      None => {}
    }
  }
}
