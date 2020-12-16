use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Instruction {
  North(isize),
  East(isize),
  South(isize),
  West(isize),
  Right(isize),
  Left(isize),
  Forward(isize),
}

struct Ship {
  dir: Direction,
  x: isize,
  y: isize,
}

impl Ship {
  fn apply(&mut self, inst: &Instruction) {
    match inst {
      Instruction::North(value) => self.y += value,
      Instruction::East(value) => self.x += value,
      Instruction::South(value) => self.y -= value,
      Instruction::West(value) => self.x -= value,
      Instruction::Right(value) => self.turn(*value),
      Instruction::Left(value) => self.turn(-*value),
      Instruction::Forward(value) => self.forward(*value),
    }
  }

  fn turn(&mut self, degrees: isize) {
    if degrees % 90 != 0 {
      panic!("Turn degrees not a multiple of 90")
    }
    let mut steps = degrees / 90;
    while steps < 0 {
      steps += 4;
    }
    while steps > 0 {
      self.dir = match self.dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
      };
      steps -= 1;
    }
  }

  fn forward(&mut self, value: isize) {
    match self.dir {
      Direction::North => self.y += value,
      Direction::East => self.x += value,
      Direction::South => self.y -= value,
      Direction::West => self.x -= value,
    }
  }
}

fn to_instruction(s: &str) -> Instruction {
  lazy_static! {
    static ref INST_RE: Regex = Regex::new(r"^(\w)(\d+)$")
      .expect("Failed to compile instruction regex");
  }
  match INST_RE.captures(s) {
    Some(cap) => {
      let value = isize::from_str_radix(&cap[2], 10).expect("Invalid instruction value");
      match &cap[1] {
        "N" => Instruction::North(value),
        "E" => Instruction::East(value),
        "S" => Instruction::South(value),
        "W" => Instruction::West(value),
        "R" => Instruction::Right(value),
        "L" => Instruction::Left(value),
        "F" => Instruction::Forward(value),
        _ => panic!("Invalid instruction type"),
      }
    }
    None => panic!("Line does not match instruction regex")
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut ship = Ship{
    dir: Direction::East,
    x: 0,
    y: 0,
  };
  let instructions = lines.map(|x| to_instruction(&x));
  for inst in instructions {
    ship.apply(&inst);
  }
  let dx = ship.x.abs();
  let dy = ship.y.abs();
  println!("Ship moved {} + {} = {}", dx, dy, dx + dy);
}
