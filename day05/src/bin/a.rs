use std::cmp;
use std::io::{BufRead,BufReader};
use std::fs::File;

struct Seat {
  row: usize,
  column: usize,
}

impl Seat {
  fn id(&self) -> usize {
    self.row * 8 + self.column
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut seats = Vec::new();
  for line in lines {
    let mut seat = Seat{row: 0, column: 0};
    for c in line.chars() {
      match c {
        'F' => { seat.row = seat.row << 1; },
        'B' => { seat.row = (seat.row << 1) + 1; },
        'L' => { seat.column = seat.column << 1; },
        'R' => { seat.column = (seat.column << 1) + 1; },
        _ => panic!("Unexpected input character")
      }
    }
    seats.push(seat);
  }

  let max_id = seats.iter().map(|seat| seat.id()).fold(0, cmp::max);
  println!("Maximum id: {}", max_id);
}
